use crate::{groups::traits::{Element, Group, Scalar}, keys::PublicKey, Ciphertext, ShuffleProof};
use rand::random_range;
use std::sync::Arc;
use sha2::{Digest, Sha256};

pub struct Shuffler<G: Group> {
    group: Arc<G>,
    h_list: Vec<G::Element>,
    pk: PublicKey<G>,
    n: usize,
}

impl<G: Group> Shuffler<G> {
    pub fn new(group: Arc<G>, h_list: Vec<G::Element>, pk: PublicKey<G>) -> Self {
        let n = h_list.len();
        Self {
            group,
            h_list,
            pk,
            n,
        }
    }

    pub fn gen_permutation(&self) -> Vec<usize> {
        let mut i_aux: Vec<usize> = (0..self.n).collect();
        let mut psi = Vec::new();

        for i in 0..self.n {
            let k = random_range(i..self.n);
            psi.push(i_aux[k]);
            i_aux[k] = i_aux[i];
        }

        return psi
    }

    pub fn gen_shuffle(&self, e_list: &[Ciphertext<G>]) -> (Vec<Ciphertext<G>>, Vec<G::Scalar>, Vec<usize>) {
        assert_eq!(e_list.len(), self.n, "e_list must have size {}", self.n);

        let mut e_prime_list = Vec::new();
        let mut e_prime_tmp = Vec::new();
        let mut r_prime_list = Vec::new();
        let psi = self.gen_permutation();

        for i in 0..self.n {
            let Ciphertext(a, b) = &e_list[i];

            let r_prime = self.group.random_scalar();
            let a_prime = a.add(&self.pk.element.mul_scalar(&r_prime));
            let b_prime = b.add(&self.group.mul_generator(&r_prime));
            let e_prime = Ciphertext(a_prime, b_prime);

            e_prime_tmp.push(e_prime);
            r_prime_list.push(r_prime);
        }

        for i in 0..self.n {
            e_prime_list.push(e_prime_tmp[psi[i]].clone());
        }

        return (e_prime_list, r_prime_list, psi)
    }

    pub fn gen_commitment(&self, psi: &[usize]) -> (Vec<G::Element>, Vec<G::Scalar>) {
        assert_eq!(psi.len(), self.n, "psi must have size {}", self.n);

        let mut r_list = vec![self.group.zero(); self.n];
        let mut c_list = vec![self.group.identity(); self.n];

        for i in 0..self.n {
            let r = self.group.random_scalar();
            let c = self.group.mul_generator(&r).add(&self.h_list[i]);

            r_list[psi[i]] = r;
            c_list[psi[i]] = c;
        }
        
        return (c_list, r_list)
    }

    pub fn gen_commitment_chain(&self, c0: &G::Element, u_list: &[G::Scalar]) -> (Vec<G::Element>, Vec<G::Scalar>) {
        let mut r_list = Vec::new();
        let mut c_list: Vec<G::Element> = Vec::new();

        for i in 0..self.n {
            let r = self.group.random_scalar();
            let c = self.group.mul_generator(&r).add(&(if i == 0 {c0.clone()} else {c_list[i-1].clone()}).mul_scalar(&u_list[i]));

            r_list.push(r);
            c_list.push(c);
        }

        return (c_list, r_list)
    }

    pub fn gen_proof(
        &self,
        e_list: &[Ciphertext<G>],
        e_prime_list: &[Ciphertext<G>],
        r_prime_list: &[G::Scalar],
        psi: &[usize]
    ) -> ShuffleProof<G> {
        let (c_list, r_list) = self.gen_commitment(psi);
        let mut u_list = Vec::new();

        for i in 0..self.n {
            // IMPORTANTE
            // TODO: definir forma canonica de serializacao para hash com formato consistente
            u_list.push(self.group.scalar_from_bytes(&Sha256::digest(format!("(({:?},{:?},{:?}),{:?})", e_list, e_prime_list, c_list, i).replace(" ", "").as_bytes())));
        }

        let u_prime_list: Vec<G::Scalar> = psi.iter().map(|&i| u_list[i].clone()).collect();

        let (c_hat_list, r_hat_list) = self.gen_commitment_chain(&self.h_list[0], &u_prime_list);

        let mut r_bar = self.group.zero();
        for i in 0..self.n {
            r_bar = r_bar.add(&r_list[i]);
        }

        let mut v_list = vec![self.group.zero(); self.n];
        v_list[self.n - 1] = self.group.one();
        for i in (0..self.n-1).rev() {
            v_list[i] = u_prime_list[i+1].mul(&v_list[i+1]);
        }

        let mut r_hat = self.group.zero();
        let mut r_tilde = self.group.zero();
        let mut r_prime = self.group.zero();
        for i in 0..self.n {
            r_hat = r_hat.add(&r_hat_list[i].mul(&v_list[i]));
            r_tilde = r_tilde.add(&r_list[i].mul(&u_list[i]));
            r_prime = r_prime.add(&r_prime_list[i].mul(&u_list[i]));
        }

        let w_list: Vec<G::Scalar> = (0..4).map(|_| self.group.random_scalar()).collect();
        let w_hat_list: Vec<G::Scalar> = (0..self.n).map(|_| self.group.random_scalar()).collect();
        let w_prime_list: Vec<G::Scalar> = (0..self.n).map(|_| self.group.random_scalar()).collect();

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

        let mut t_hat_list = Vec::new();
        for i in 0..self.n {
            t_hat_list.push(self.group.mul_generator(&w_hat_list[i]).add(&(if i == 0 {self.h_list[0].clone()} else {c_hat_list[i-1].clone()}).mul_scalar(&w_prime_list[i])));
        }

        let y = (e_list, e_prime_list, c_list.clone(), c_hat_list.clone(), self.pk.element.clone());
        let t = (t0, t1, t2, (t3_0, t3_1), t_hat_list);
        // IMPORTANTE
        // TODO: definir forma canonica de serializacao para hash com formato consistente
        let c = self.group.scalar_from_bytes(&Sha256::digest(format!("({:?},{:?})", y, t).replace(" ", "").as_bytes()));

        let s0 = w_list[0].add(&c.mul(&r_bar));
        let s1 = w_list[1].add(&c.mul(&r_hat));
        let s2 = w_list[2].add(&c.mul(&r_tilde));
        let s3 = w_list[3].add(&c.mul(&r_prime));

        let mut s_hat_list = Vec::new();
        let mut s_prime_list = Vec::new();
        for i in 0..self.n {
            s_hat_list.push(w_hat_list[i].add(&c.mul(&r_hat_list[i])));
            s_prime_list.push(w_prime_list[i].add(&c.mul(&u_prime_list[i])));
        }
        let s = (s0, s1, s2, s3, s_hat_list, s_prime_list);
        return ShuffleProof(t, s, c_list, c_hat_list)
    }
}