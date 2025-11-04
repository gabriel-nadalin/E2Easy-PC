use crate::{Scalar, Element, G, utils::*, types::ShuffleProof};
use sha2::{Digest, Sha256};
use rand::random_range;

pub struct Shuffler {
    h_list: Vec<Element>,
    n: usize,
}

impl Shuffler {
    pub fn new(h_list: Vec<Element>) -> Self {
        let n = h_list.len();
        Self {
            h_list,
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

    pub fn gen_shuffle(&self, commit_list: &Vec<Element>) -> (Vec<Element>, Vec<Scalar>, Vec<usize>) {
        assert_eq!(commit_list.len(), self.n, "commit_list must have size {}", self.n);

        let mut recommit_list = Vec::new();
        let mut recommit_tmp  = Vec::new();
        let mut r_prime_list  = Vec::new();
        let psi = self.gen_permutation();

        for i in 0..self.n {
            let r_prime = random_scalar();
            let c0 = G * r_prime; // I guess no need to multiply for h^0, since it is 1, right?
            let recommit = commit_list[i] + c0;

            recommit_tmp.push(recommit);
            r_prime_list.push(r_prime);
        }

        for i in 0..self.n {
            recommit_list.push(recommit_tmp[psi[i]].clone());
        }

        return (recommit_list, r_prime_list, psi)
    }

    pub fn gen_commitment(&self, psi: &[usize]) -> (Vec<Element>, Vec<Scalar>) {
        assert_eq!(psi.len(), self.n, "psi must have size {}", self.n);

        let mut r_list = vec![Scalar::ZERO; self.n];
        let mut c_list = vec![Element::IDENTITY; self.n];

        for i in 0..self.n {
            let r = random_scalar();
            let c = (G * r) + self.h_list[i];

            r_list[psi[i]] = r;
            c_list[psi[i]] = c;
        }
        
        return (c_list, r_list)
    }

    pub fn gen_commitment_chain(&self, c0: &Element, u_list: &[Scalar]) -> (Vec<Element>, Vec<Scalar>) {
        assert_eq!(u_list.len(), self.n, "u_list must have size {}", self.n);

        let mut r_list = Vec::new();
        let mut c_list: Vec<Element> = Vec::new();

        for i in 0..self.n {
            let r = random_scalar();
            let c: Element;
            if i == 0 {
                c = (G * r) + (*c0 * u_list[i]);
            } else {
                c = (G * r) + (c_list[i-1] * u_list[i]);
            }

            r_list.push(r);
            c_list.push(c);
        }

        return (c_list, r_list)
    }

    pub fn gen_proof(
        &self,
        commit_list: &Vec<Element>,
        commit_prime_list: &Vec<Element>,
        r_prime_list: &Vec<Scalar>,
        psi: &[usize]
    ) -> ShuffleProof {
        assert_eq!(commit_list.len(), self.n, "commit_list must have size {}", self.n);
        assert_eq!(commit_prime_list.len(), self.n, "commit_prime_list must have size {}", self.n);
        assert_eq!(r_prime_list.len(), self.n, "r_prime_list must have size {}", self.n);
        assert_eq!(psi.len(), self.n, "psi must have size {}", self.n);
        
        let (c_list, r_list) = self.gen_commitment(psi);
        let mut u_list = Vec::new();

        for i in 0..self.n {
            // IMPORTANTE
            // TODO: definir forma canonica de serializacao para hash com formato consistente
            u_list.push(scalar_from_bytes(&Sha256::digest(format!("(({:?},{:?},{:?}),{:?})", commit_list, commit_prime_list, c_list, i).replace(" ", "").as_bytes())));
        }
        let u_prime_list: Vec<Scalar> = (0..self.n).map(|i| u_list[psi[i]]).collect();

        let mut v_list = vec![Scalar::ZERO; self.n];
        v_list[self.n - 1] = Scalar::ONE;
        for i in (0..self.n-1).rev() {
            v_list[i] = u_prime_list[i+1] * v_list[i+1];
        }

        let (c_hat_list, r_hat_list) = self.gen_commitment_chain(&self.h_list[0], &u_prime_list);

        let mut r_bar = Scalar::ZERO;
        let mut r_hat = Scalar::ZERO;
        let mut r_tilde = Scalar::ZERO;
        let mut r_prime = Scalar::ZERO;
        for i in 0..self.n {
            r_bar   += r_list[i];
            r_hat   += r_hat_list[i]   * v_list[i];
            r_tilde += r_list[i]       * u_list[i];
            r_prime += r_prime_list[i] * u_list[i];
        }

        let w_list:       Vec<Scalar> = (0..4)     .map(|_| random_scalar()).collect();
        let w_hat_list:   Vec<Scalar> = (0..self.n).map(|_| random_scalar()).collect();
        let w_prime_list: Vec<Scalar> = (0..self.n).map(|_| random_scalar()).collect();

        let t0: Element = G * w_list[0];
        let t1: Element = G * w_list[1];
        let t2: Element = summation((0..self.n).map(|i| self.h_list[i]       * w_prime_list[i]).collect()) + (G * w_list[2]);
        let t3: Element = summation((0..self.n).map(|i| commit_prime_list[i] * w_prime_list[i]).collect()) - (G * w_list[3]);

        let mut t_hat_list = Vec::new();
        for i in 0..self.n {
            if i == 0 {
                t_hat_list.push((G * w_hat_list[i]) + (self.h_list[0]  * w_prime_list[i]));
            } else {
                t_hat_list.push((G * w_hat_list[i]) + (c_hat_list[i-1] * w_prime_list[i]));
            }
        }

        let y = (commit_list, commit_prime_list, c_list.clone(), c_hat_list.clone());
        let t = (t0, t1, t2, t3, t_hat_list);
        // IMPORTANTE
        // TODO: definir forma canonica de serializacao para hash com formato consistente
        let c = scalar_from_bytes(&Sha256::digest(format!("({:?},{:?})", y, t).replace(" ", "").as_bytes()));

        let s0: Scalar = w_list[0] + (c * &r_bar);
        let s1: Scalar = w_list[1] + (c * &r_hat);
        let s2: Scalar = w_list[2] + (c * &r_tilde);
        let s3: Scalar = w_list[3] + (c * &r_prime);

        let mut s_hat_list: Vec<Scalar> = Vec::new();
        let mut s_prime_list: Vec<Scalar> = Vec::new();
        for i in 0..self.n {
            s_hat_list  .push(w_hat_list[i]   + (c * r_hat_list[i]));
            s_prime_list.push(w_prime_list[i] + (c * u_prime_list[i]));
        }
        let s = (s0, s1, s2, s3, s_hat_list, s_prime_list);
        return ShuffleProof::new(t, s, c_list, c_hat_list)
    }
}
