use crate::{groups::{Element, Scalar}, keys::PublicKey, *};
use core::array::from_fn;
use std::sync::Arc;
use sha2::{Digest, Sha256};

pub struct Verifier<G: Group> {
    group: Arc<G>,
    h_list: [G::Element; N],
}

impl<G: Group> Verifier<G> {
    pub fn new(group: Arc<G>, h_list: [G::Element; N]) -> Self {
        Self {
            group,
            h_list,
        }
    }

    pub fn check_proof(&self, pi: ShuffleProof<G>, e_list: [Ciphertext<G>; N], e_prime_list: [Ciphertext<G>; N], pk: PublicKey<G>) -> bool {
        let ShuffleProof(t, s, c_list, c_hat_list) = pi;

        let mut u_list: [<G as Group>::Scalar; N] = from_fn(|_| self.group.zero());
        for i in 0..N {
            // IMPORTANTE
            // TODO: definir forma canonica de serializacao para hash com formato consistente
            u_list[i] = self.group.scalar_from_bytes(&Sha256::digest(format!("(({:?},{:?},{:?}),{:?})", e_list, e_prime_list, c_list, i).replace(" ", "").as_bytes()));
        }

        let c_bar = c_list.iter().fold(self.group.identity(), |acc, x| acc.add(x))
            .add(&self.h_list.iter().fold(self.group.identity(), |acc, x| acc.add(x)).inv());
        let u = u_list.iter().fold(self.group.one(), |acc, x| acc.mul(x));

        let c_hat = c_hat_list[N-1].add(&self.h_list[0].mul_scalar(&u).inv());
        let c_tilde = c_list.iter()
            .zip(u_list.iter())
            .map(|(c, u)| c.mul_scalar(u))
            .fold(self.group.identity(), |acc, x| acc.add(&x));
        let a_prime = e_list.iter()
            .zip(u_list.iter())
            .map(|(e, u)| e.0.mul_scalar(u))
            .fold(self.group.identity(), |acc, x| acc.add(&x));
        let b_prime = e_list.iter()
            .zip(u_list.iter())
            .map(|(e, u)| e.1.mul_scalar(u))
            .fold(self.group.identity(), |acc, x| acc.add(&x));

        let y = (e_list, e_prime_list.clone(), c_list, c_hat_list.clone(), pk.element.clone());
        // IMPORTANTE
        // TODO: definir forma canonica de serializacao para hash com formato consistente
        let c = self.group.scalar_from_bytes(&Sha256::digest(format!("({:?}, {:?})", y, t).replace(" ", "").as_bytes()));

        let t_prime_0 = c_bar.mul_scalar(&c).inv().add(&self.group.mul_generator(&s.0));
        let t_prime_1 = c_hat.mul_scalar(&c).inv().add(&self.group.mul_generator(&s.1));
        let t_prime_2 = c_tilde.mul_scalar(&c).inv().add(&self.group.mul_generator(&s.2)).add(
            &self.h_list.iter()
            .zip(s.5.iter())
            .map(|(h, s_prime)| h.mul_scalar(s_prime))
            .fold(self.group.identity(), |acc, x| acc.add(&x))
        );
        let t_prime_3_0 = a_prime.mul_scalar(&c).inv().add(&pk.element.mul_scalar(&s.3).inv()).add(
            &e_prime_list.iter()
            .zip(s.5.iter())
            .map(|(e, s_prime)| e.0.mul_scalar(s_prime))
            .fold(self.group.identity(), |acc, x| acc.add(&x))
        );
        let t_prime_3_1 = b_prime.mul_scalar(&c).inv().add(&self.group.mul_generator(&s.3).inv()).add(
            &e_prime_list.iter()
            .zip(s.5.iter())
            .map(|(e, s_prime)| e.1.mul_scalar(s_prime))
            .fold(self.group.identity(), |acc, x| acc.add(&x))
        );

        let mut t_hat_prime_list = from_fn(|_| self.group.identity());
        for i in 0..N {
            t_hat_prime_list[i] = c_hat_list[i].mul_scalar(&c).inv().add(&self.group.mul_generator(&s.4[i])).add(&(if i == 0 {&self.h_list[0]} else {&c_hat_list[i-1]}).mul_scalar(&s.5[i]));
        }

        let t_prime = (t_prime_0, t_prime_1, t_prime_2, (t_prime_3_0, t_prime_3_1), t_hat_prime_list);
        // println!("{:#?}", t_prime);
        return t == t_prime
    }
}