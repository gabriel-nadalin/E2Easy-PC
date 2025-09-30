use rand::random_range;
use core::array::from_fn;
use crate::{Ciphertext, Proof, N, Number, NumberNZ, ModNumber, ModNumberParams, utils::{get_exponent, prod, hash, modnumber_to_number, ciphertext_to_number}};

pub struct Shuffler {
    p: ModNumberParams,
    q: NumberNZ,
    g: ModNumber,
    h_list: [ModNumber; N],
    pk: ModNumber
}

impl Shuffler {
    pub fn new(p: ModNumberParams, q: NumberNZ, g: ModNumber, h_list: [ModNumber; N], pk: ModNumber) -> Self {
        Self {
            p,
            q,
            g,
            h_list,
            pk
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

    pub fn gen_shuffle(&self, e_list: [Ciphertext; N]) -> ([Ciphertext; N], [Number; N], [usize; N]) {
        let mut e_prime_list = [(ModNumber::zero(self.p), ModNumber::zero(self.p)); N];
        let mut e_prime_tmp = [(ModNumber::zero(self.p), ModNumber::zero(self.p)); N];
        let mut r_prime_list = [Number::ZERO; N];
        let psi = Self::gen_permutation();

        for i in 0..N {
            let (a, b) = e_list[i];

            let r_prime = get_exponent(&self.q).unwrap();
            let a_prime = a.mul(&self.pk.pow(&r_prime));
            let b_prime = b.mul(&self.g.pow(&r_prime));
            let e_prime = (a_prime, b_prime);

            e_prime_tmp[i] = e_prime;
            r_prime_list[i] = r_prime;
        }

        for i in 0..N {
            e_prime_list[i] = e_prime_tmp[psi[i]];
        }
        
        return (e_prime_list, r_prime_list, psi)
    }

    pub fn gen_commitment(&self, psi: [usize; N]) -> ([ModNumber; N], [Number; N]) {
        let mut r_list = [Number::ZERO; N];
        let mut c_list = [ModNumber::zero(self.p); N];

        for i in 0..N {
            let r = get_exponent(&self.q).unwrap();
            let c = self.h_list[i].mul(&self.g.pow(&r));
            r_list[psi[i]] = r;
            c_list[psi[i]] = c;
        }

        return (c_list, r_list)
    }

    pub fn gen_commitment_chain(&self, c0: ModNumber, u_list: [Number; N]) -> ([ModNumber; N], [Number; N]) {
        let mut r_list = [Number::ZERO; N];
        let mut c_list = [ModNumber::zero(self.p); N];

        for i in 0..N {
            let r = get_exponent(&self.q).unwrap();
            let c = self.g.pow(&r).mul(&(if i == 0 {c0} else {c_list[i-1]}).pow(&u_list[i]));
            r_list[i] = r;
            c_list[i] = c;
        }

        return (c_list, r_list)
    }

    pub fn gen_proof(&self, e_list: [Ciphertext; N], e_prime_list: [Ciphertext; N], r_prime_list: [Number; N], psi: [usize; N]) -> Proof {
        let (c_list, r_list) = self.gen_commitment(psi);
        let mut u_list = [Number::ZERO; N]; // u64

        for i in 0..N {
            u_list[i] = hash(((e_list[i].0.retrieve(), e_list[i].1.retrieve(), e_prime_list[i].0.retrieve(), e_prime_list[i].1.retrieve(), c_list[i].retrieve()), i));
        }
        let u_prime_list: [Number; N] = from_fn(|i| u_list[psi[i]]);

        let (c_hat_list, r_hat_list) = self.gen_commitment_chain(self.h_list[0], u_prime_list);

        let mut r_bar = Number::ZERO;
        for i in 0..N {
            r_bar = r_bar.add_mod(&(&r_list[i]).into(), &self.q);
        }

        let mut v_list = [Number::ZERO; N];
        v_list[N - 1] = Number::ONE;
        for i in (0..N-1).rev() {
            v_list[i] = v_list[i+1].mul_mod(&u_prime_list[i+1], &self.q);
        }

        let mut r_hat = Number::ZERO;
        let mut r_tilde = Number::ZERO; 
        let mut r_prime = Number::ZERO;
        for i in 0..N {
            r_hat = r_hat.add_mod(&r_hat_list[i].mul_mod(&v_list[i], &self.q), &self.q);
            r_tilde = r_tilde.add_mod(&r_list[i].mul_mod(&u_list[i], &self.q), &self.q);
            r_prime = r_prime.add_mod(&r_prime_list[i].mul_mod(&u_list[i], &self.q), &self.q);
        }

        let w_list: [Number; 4] = from_fn(|_| get_exponent(&self.q).unwrap());
        let w_hat_list: [Number; N] = from_fn(|_| get_exponent(&self.q).unwrap());
        let w_prime_list: [Number; N] = from_fn(|_| get_exponent(&self.q).unwrap());

        let t0 = self.g.pow(&w_list[0]);
        let t1 = self.g.pow(&w_list[1]);
        let t2 = self.g.pow(&w_list[2]).mul(&prod(from_fn(|i| self.h_list[i].pow(&w_prime_list[i])), self.p));
        let t3_0 = self.pk.pow(&w_list[3]).inv().unwrap().mul(&prod(from_fn(|i| e_prime_list[i].0.pow(&w_prime_list[i])), self.p));
        let t3_1 = self.g.pow(&w_list[3]).inv().unwrap().mul(&prod(from_fn(|i| e_prime_list[i].1.pow(&w_prime_list[i])), self.p));

        let mut t_hat_list = [ModNumber::zero(self.p); N];
        for i in 0..N {
            t_hat_list[i] = self.g.pow(&w_hat_list[i]).mul(&(if i == 0 {self.h_list[0]} else {c_hat_list[i-1]}).pow(&w_prime_list[i]));
        }

        let y = (ciphertext_to_number(e_list), ciphertext_to_number(e_prime_list), modnumber_to_number(c_list), modnumber_to_number(c_hat_list), self.pk.retrieve());
        let t = (t0, t1, t2, (t3_0, t3_1), t_hat_list);
        let temp_t = (t0.retrieve(), t1.retrieve(), t2.retrieve(), (t3_0.retrieve(), t3_1.retrieve()), modnumber_to_number(t_hat_list));
        let c = hash((y, temp_t));

        let s0 = w_list[0].add_mod(&c.mul_mod(&r_bar, &self.q), &self.q);
        let s1 = w_list[1].add_mod(&c.mul_mod(&r_hat, &self.q), &self.q);
        let s2 = w_list[2].add_mod(&c.mul_mod(&r_tilde, &self.q), &self.q);
        let s3 = w_list[3].add_mod(&c.mul_mod(&r_prime, &self.q), &self.q);
        /*
        println!("g^r--: {:?}", self.g.pow(&r_bar).retrieve());
        println!("(g^r--)^⁻c: {:?}", self.g.pow(&r_bar).pow(&c).inv().unwrap().retrieve());
        println!("r--: {:?}", r_bar);
        println!("q: {:?}", self.q);
        let c_bar = prod(c_list, self.p).mul(&prod(self.h_list, self.p).inv().unwrap());
        println!("c--: {:?}", c_bar.retrieve());
        let t_prime_0 = self.g.pow(&s0).mul(&c_bar.pow(&c).inv().unwrap());
        println!("t_0_shuf {:?}", t0.retrieve());
        println!("g: {:?}", self.g.retrieve());
        println!("w_0: {:?}", w_list[0]);

        println!("t'_0_shuf {:?}", t_prime_0.retrieve());
        println!("g: {:?}", self.g.retrieve());
        println!("s_0: {:?}", s0);
        println!("g^s_0: {:?}", self.g.pow(&s0).retrieve());
        println!("r_bar: {:?}", r_bar);
        println!("c*r_bar: {:?}", r_bar.mul_mod(&c, &self.q));
        println!("g^(c*r_bar): {:?}", self.g.pow(&r_bar.mul_mod(&c, &self.q)).retrieve());
        println!("c_bar: {:?}", c_bar.retrieve());
        println!("c: {:?}", c);
        println!("c_bar^-c: {:?}", c_bar.pow(&c).inv().unwrap().retrieve());

        println!("t_0_shuf {:?}", t0.retrieve());
        println!("t'_0_shuf {:?}", t_prime_0.retrieve());
        */

        let mut s_hat_list = [Number::ZERO; N];
        let mut s_prime_list = [Number::ZERO; N];
        for i in 0..N {
            s_hat_list[i] = w_hat_list[i].add_mod(&c.mul_mod(&r_hat_list[i], &self.q), &self.q);
            s_prime_list[i] =  w_prime_list[i].add_mod(&c.mul_mod(&u_prime_list[i], &self.q), &self.q);
        }
        let s = (s0, s1, s2, s3, s_hat_list, s_prime_list);
        return (t, s, c_list, c_hat_list)
    }
}
