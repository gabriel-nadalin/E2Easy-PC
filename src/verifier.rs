use crate::{Scalar, Element, G, utils::*, types::ShuffleProof};
use sha2::{Digest, Sha256};

pub struct Verifier {
    h_list: Vec<Element>,
    n: usize,
}

impl Verifier {
    pub fn new(h_list: Vec<Element>) -> Self {
        let n = h_list.len();
        Self {
            h_list,
            n,
        }
    }

    pub fn check_proof(&self, pi: &ShuffleProof, commit_list: &Vec<Element>, commit_prime_list: &Vec<Element>) -> bool {
        let (t, s, c_list, c_hat_list) = pi.components();

        let mut u_list: Vec<Scalar> = Vec::new();
        for i in 0..self.n {
            // IMPORTANTE
            // TODO: definir forma canonica de serializacao para hash com formato consistente
            u_list.push(scalar_from_bytes(&Sha256::digest(format!("(({:?},{:?},{:?}),{:?})", commit_list, commit_prime_list, c_list, i).replace(" ", "").as_bytes()))); // Shoul it be commit_list[i] and similar?
        }

        let c_bar = c_list.iter().fold(Element::IDENTITY, |acc, x| acc + x) - self.h_list.iter().fold(Element::IDENTITY, |acc, x| acc + x);
        let u = u_list.iter().fold(Scalar::ONE, |acc, x| acc * x);

        let c_hat = c_hat_list[self.n-1] - (self.h_list[0] * u);
        let c_tilde = c_list.iter()
            .zip(u_list.iter())
            .map(|(c, u)| c * u)
            .fold(Element::IDENTITY, |acc, x| acc + x); // I feel like it can be done without id sum...
        let e_prime = commit_list.iter()
            .zip(u_list.iter())
            .map(|(e, u)| e * u)
            .fold(Element::IDENTITY, |acc, x| acc + x); // I feel like it can be done without id sum...

        let y = (commit_list, commit_prime_list, c_list, c_hat_list.clone());
        // IMPORTANTE
        // TODO: definir forma canonica de serializacao para hash com formato consistente
        let c = scalar_from_bytes(&Sha256::digest(format!("({:?}, {:?})", y, t).replace(" ", "").as_bytes()));

        let t_prime_0 = (G * s.0) - (c_bar * c);
        let t_prime_1 = (G * s.1) - (c_hat * c);
        let t_prime_2 = -(c_tilde * c) + (G *s.2) + self.h_list.iter()
            .zip(s.5.iter())
            .map(|(h, s_prime)| h * s_prime)
            .fold(Element::IDENTITY, |acc, x| acc + x); // I feel like it can be done without id sum...
        let t_prime_3 = -(e_prime * c) - (G * s.3) + commit_prime_list.iter()
            .zip(s.5.iter())
            .map(|(e, s_prime)| e * s_prime)
            .fold(Element::IDENTITY, |acc, x| acc + x); // I feel like it can be done without id sum...

        let mut t_hat_prime_list = Vec::new();
        for i in 0..self.n {
            if i == 0 {
                t_hat_prime_list.push((G * s.4[i]) + (self.h_list[0] * s.5[i]) - (c_hat_list[i] * c));
            } else {
                t_hat_prime_list.push((G * s.4[i]) + (c_hat_list[i-1] * s.5[i]) - (c_hat_list[i] * c));
            }
        }

        let t_prime = (t_prime_0, t_prime_1, t_prime_2, t_prime_3, t_hat_prime_list);
        // println!("{:#?}", t_prime);
        return *t == t_prime
    }
}
