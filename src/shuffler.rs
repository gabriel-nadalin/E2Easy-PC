use crate::{groups::{Element, Group, Scalar}, keys::PublicKey, Ciphertext, ShuffleProof, N};
use rand::random_range;
use core::array::from_fn;
use std::sync::Arc;
use sha2::{Digest, Sha256};

pub struct Shuffler<G: Group> {
    group: Arc<G>,
    h_list: [G::Element; N],
    pk: PublicKey<G>,
}

impl<G: Group> Shuffler<G> {
    pub fn new(group: Arc<G>, h_list: [G::Element; N], pk: PublicKey<G>) -> Self {
        Self {
            group,
            h_list,
            pk,
        }
    }

    pub fn gen_permutation() -> [usize; N] {
        let mut i_aux: [usize; N] = from_fn(|i| i);
        let mut psi: [usize; N] = [0; N];

        for i in 0..N {
            let k = random_range(i..N);
            psi[i] = i_aux[k];
            i_aux[k] = i_aux[i];
        }

        return psi
    }

    pub fn gen_shuffle(&self, e_list: [Ciphertext<G>; N]) -> ([Ciphertext<G>; N], [G::Scalar; N], [usize; N]) {
        let mut e_prime_list = from_fn(|_| Ciphertext(self.group.identity(), self.group.identity()));
        let mut e_prime_tmp: [Ciphertext<G>; N] = from_fn(|_| Ciphertext(self.group.identity(), self.group.identity()));
        let mut r_prime_list = from_fn(|_| self.group.zero());
        let psi = Self::gen_permutation();

        for i in 0..N {
            let Ciphertext(a, b) = &e_list[i];

            let r_prime = self.group.random_scalar();
            let a_prime = a.add(&self.pk.element.mul_scalar(&r_prime));
            let b_prime = b.add(&self.group.mul_generator(&r_prime));
            let e_prime = Ciphertext(a_prime, b_prime);

            e_prime_tmp[i] = e_prime;
            r_prime_list[i] = r_prime;
        }

        for i in 0..N {
            e_prime_list[i] = e_prime_tmp[psi[i]].clone();
        }

        return (e_prime_list, r_prime_list, psi)
    }

    pub fn gen_commitment(&self, psi: [usize; N]) -> ([G::Element; N], [G::Scalar; N]) {
        let mut r_list = from_fn(|_| self.group.zero());
        let mut c_list = from_fn(|_| self.group.identity());

        for i in 0..N {
            let r = self.group.random_scalar();
            let c = self.group.mul_generator(&r).add(&self.h_list[i]);

            r_list[psi[i]] = r;
            c_list[psi[i]] = c;
        }
        
        return (c_list, r_list)
    }

    pub fn gen_commitment_chain(&self, c0: &G::Element, u_list: &[G::Scalar; N]) -> ([G::Element; N], [G::Scalar; N]) {
        let mut r_list = from_fn(|_| self.group.zero());
        let mut c_list = from_fn(|_| self.group.identity());

        for i in 0..N {
            let r = self.group.random_scalar();
            let c = self.group.mul_generator(&r).add(&(if i == 0 {c0.clone()} else {c_list[i-1].clone()}).mul_scalar(&u_list[i]));

            r_list[i] = r;
            c_list[i] = c;
        }

        return (c_list, r_list)
    }

    pub fn gen_proof(
        &self,
        e_list: [Ciphertext<G>; N],
        e_prime_list: [Ciphertext<G>; N],
        r_prime_list: [G::Scalar; N],
        psi: [usize; N]
    ) -> ShuffleProof<G> {
        let (c_list, r_list) = self.gen_commitment(psi);
        let mut u_list: [<G as Group>::Scalar; N] = from_fn(|_| self.group.zero());

        for i in 0..N {
            // IMPORTANTE
            // TODO: definir forma canonica de serializacao para hash com formato consistente
            u_list[i] = self.group.deserialize_to_scalar(Sha256::digest(format!("(({:?},{:?},{:?}),{:?})", e_list, e_prime_list, c_list, i).replace(" ", "").as_bytes()).to_vec());
        }

        let u_prime_list = from_fn(|i| u_list[psi[i]].clone());

        let (c_hat_list, r_hat_list) = self.gen_commitment_chain(&self.h_list[0], &u_prime_list);

        let mut r_bar = self.group.zero();
        for i in 0..N {
            r_bar = r_bar.add(&r_list[i]);
        }

        let mut v_list: [<G as Group>::Scalar; N] = from_fn(|_| self.group.zero());
        v_list[N - 1] = self.group.one();
        for i in (0..N-1).rev() {
            v_list[i] = u_prime_list[i+1].mul(&v_list[i+1]);
        }

        let mut r_hat = self.group.zero();
        let mut r_tilde = self.group.zero();
        let mut r_prime = self.group.zero();
        for i in 0..N {
            r_hat = r_hat.add(&r_hat_list[i].mul(&v_list[i]));
            r_tilde = r_tilde.add(&r_list[i].mul(&u_list[i]));
            r_prime = r_prime.add(&r_prime_list[i].mul(&u_list[i]));
        }

        let w_list: [G::Scalar; 4] = from_fn(|_| self.group.random_scalar());
        let w_hat_list: [G::Scalar; N] = from_fn(|_| self.group.random_scalar());
        let w_prime_list: [G::Scalar; N] = from_fn(|_| self.group.random_scalar());

        let t0 = self.group.mul_generator(&w_list[0]);
        let t1 = self.group.mul_generator(&w_list[1]);
        let t2 = self.group.mul_generator(&w_list[2]).add(
            &self.h_list
                .iter()
                .zip(w_prime_list.iter())
                .map(|(h, w_prime)| h.mul_scalar(w_prime))   // each h_i^{w'_i}
                .fold(self.group.identity(), |acc, x| acc.add(&x))
        );
        let t3_0 = self.pk.element.mul_scalar(&w_list[3]).inv().add(
            &e_prime_list
                .iter()
                .zip(w_prime_list.iter())
                .map(|(e_prime, w_prime)| e_prime.0.mul_scalar(w_prime))
                .fold(self.group.identity(), |acc, x| acc.add(&x))
        );
        let t3_1 = self.group.mul_generator(&w_list[3]).inv().add(
            &e_prime_list
                .iter()
                .zip(w_prime_list.iter())
                .map(|(e_prime, w_prime)| e_prime.1.mul_scalar(w_prime))
                .fold(self.group.identity(), |acc, x| acc.add(&x))
        );

        let mut t_hat_list = from_fn(|_| self.group.identity());
        for i in 0..N {
            t_hat_list[i] = self.group.mul_generator(&w_hat_list[i]).add(&(if i == 0 {self.h_list[0].clone()} else {c_hat_list[i-1].clone()}).mul_scalar(&w_prime_list[i]));
        }

        let y = (e_list, e_prime_list, c_list.clone(), c_hat_list.clone(), self.pk.element.clone());
        let t = (t0, t1, t2, (t3_0, t3_1), t_hat_list);
        // IMPORTANTE
        // TODO: definir forma canonica de serializacao para hash com formato consistente
        let c = self.group.deserialize_to_scalar(Sha256::digest(format!("({:?},{:?})", y, t).replace(" ", "").as_bytes()).to_vec());

        let s0 = w_list[0].add(&c.mul(&r_bar));
        let s1 = w_list[1].add(&c.mul(&r_hat));
        let s2 = w_list[2].add(&c.mul(&r_tilde));
        let s3 = w_list[3].add(&c.mul(&r_prime));

        let mut s_hat_list = from_fn(|_| self.group.zero());
        let mut s_prime_list = from_fn(|_| self.group.zero());
        for i in 0..N {
            s_hat_list[i] = w_hat_list[i].add(&c.mul(&r_hat_list[i]));
            s_prime_list[i] = w_prime_list[i].add(&c.mul(&u_prime_list[i]));
        }
        let s = (s0, s1, s2, s3, s_hat_list, s_prime_list);
        return ShuffleProof(t, s, c_list, c_hat_list)
    }
}