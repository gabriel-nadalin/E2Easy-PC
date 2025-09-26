use crate::{utils::*, *};
use core::array::from_fn;

pub struct Verifier {
    p: ModNumberParams,
    q: NumberNZ,
    g: ModNumber,
    h_list: [ModNumber; N]
}

impl Verifier {
    pub fn new(p: ModNumberParams, q: NumberNZ, g: ModNumber, h_list: [ModNumber; N]) -> Self {
        Self {
            p,
            q,
            g,
            h_list
        }
    }

    pub fn check_proof(&self, pi: Proof, e_list: [Ciphertext; N], e_prime_list: [Ciphertext; N], pk: ModNumber) -> bool {
        let (t, s, c_list, c_hat_list) = pi; // t: ModNumbers, s: Numbers, c_list: ModNumbers, c_hat_list: ModNumbers

        let mut u_list = [Number::ZERO; N];
        for i in 0..N {
            u_list[i] = hash(((e_list[i].0.retrieve(), e_list[i].1.retrieve(), e_prime_list[i].0.retrieve(), e_prime_list[i].1.retrieve(), c_list[i].retrieve()), i));
        }

        let c_bar = prod(c_list, self.p).mul(&prod(self.h_list, self.p).inv().unwrap());
        let mut u = Number::ONE;
        for i in 0..N {
            u = u.mul_mod(&u_list[i], &self.q);
        }

        let c_hat = c_hat_list[N-1].mul(&self.h_list[0].pow(&u).inv().unwrap());
        let c_tilde = prod(from_fn(|i| c_list[i].pow(&u_list[i])), self.p);

        let a_prime = prod(from_fn(|i| e_list[i].0.pow(&u_list[i])), self.p);
        let b_prime = prod(from_fn(|i| e_list[i].1.pow(&u_list[i])), self.p);

        let y = (ciphertext_to_number(e_list), ciphertext_to_number(e_prime_list), modnumber_to_number(c_list), modnumber_to_number(c_hat_list), pk.retrieve());
        let temp_t = (t.0.retrieve(), t.1.retrieve(), t.2.retrieve(), (t.3.0.retrieve(), t.3.1.retrieve()), modnumber_to_number(t.4));
        let c = hash((y, temp_t));

        let t_prime_0 = self.g.pow(&s.0).mul(&c_bar.pow(&c).inv().unwrap());
        let t_prime_1 = self.g.pow(&s.1).mul(&c_hat.pow(&c).inv().unwrap());
        let t_prime_2 = self.g.pow(&s.2).mul(&c_tilde.pow(&c).inv().unwrap()).mul(&prod(from_fn(|i| self.h_list[i].pow(&s.5[i])), self.p));
        let t_prime_3_0 = pk.pow(&s.3).inv().unwrap().mul(&a_prime.pow(&c).inv().unwrap()).mul(&prod(from_fn(|i| e_prime_list[i].0.pow(&s.5[i])), self.p));
        let t_prime_3_1 = self.g.pow(&s.3).inv().unwrap().mul(&b_prime.pow(&c).inv().unwrap()).mul(&prod(from_fn(|i| e_prime_list[i].1.pow(&s.5[i])), self.p));

        let mut t_hat_prime_list = [ModNumber::zero(self.p); N];
        for i in 0..N {
            t_hat_prime_list[i] = self.g.pow(&s.4[i]).mul(&c_hat_list[i].pow(&c).inv().unwrap()).mul(&(if i == 0 {self.h_list[0]} else {c_hat_list[i-1]}).pow(&s.5[i]));
        }

        let t_prime = (t_prime_0, t_prime_1, t_prime_2, (t_prime_3_0, t_prime_3_1), t_hat_prime_list);
        /*
        let temp_t_prime = (t_prime_0.retrieve(), t_prime_1.retrieve(), t_prime_2.retrieve(), (t_prime_3_0.retrieve(), t_prime_3_1.retrieve()), modnumber_to_number(t_hat_prime_list));
        println!("t     = {:?}", temp_t);
        println!("t'    = {:?}", temp_t_prime);
        println!("t.0: {:?}", temp_t.0 == temp_t_prime.0); 
        println!("t.1: {:?}", temp_t.1 == temp_t_prime.1);
        println!("t.2: {:?}", temp_t.2 == temp_t_prime.2);
        println!("t.3: {:?}", temp_t.3 == temp_t_prime.3);
        println!("t.4: {:?}", temp_t.4 == temp_t_prime.4);
        */
        return t == t_prime
    }
}
