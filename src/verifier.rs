use crate::{Element, G, Scalar, types::{ShuffleProof, StoredElement}, utils::*};
use sha2::{Digest, Sha256};
use p256::elliptic_curve::group::prime::PrimeCurveAffine;

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
            // conversao para representacao afim eh necessaria para serializacao
            let to_hash = (
                (
                    commit_list.iter().map(|p| p.to_affine()).collect::<Vec<_>>(),
                    commit_prime_list.iter().map(|p| p.to_affine()).collect::<Vec<_>>(),
                    c_list.clone(),
                ),
                i
            );
            // IMPORTANTE
            // TODO: definir forma canonica de serializacao para hash com formato consistente
            u_list.push(scalar_from_bytes(&hash(to_hash)));
        }

        let c_bar: Element = summation(c_list.iter().map(|p| p.to_curve()).collect()) - summation(self.h_list.clone());
        // Product of u_list
        let u: Scalar = u_list.iter().fold(Scalar::ONE, |acc, x| acc * x);

        let c_hat:   Element = c_hat_list[self.n-1].to_curve() - (self.h_list[0] * u);
        let c_tilde: Element = summation((0..self.n).map(|i| c_list[i]      * u_list[i]).collect());
        let e_prime: Element = summation((0..self.n).map(|i| commit_list[i] * u_list[i]).collect());

        // conversao para representacao afim eh necessaria para serializacao
        let y = (
            commit_list.iter().map(|p| p.to_affine()).collect::<Vec<_>>(),
            commit_prime_list.iter().map(|p| p.to_affine()).collect::<Vec<_>>(),
            c_list.clone(),
            c_hat_list.clone(),
        );
        // IMPORTANTE
        // TODO: definir forma canonica de serializacao para hash com formato consistente
        let to_hash = (y, t.clone());
        let c = scalar_from_bytes(&hash(to_hash));

        let t_prime_0: StoredElement = ((G * s.0) - (c_bar * c)).to_affine();
        let t_prime_1: StoredElement = ((G * s.1) - (c_hat * c)).to_affine();
        let t_prime_2: StoredElement = (summation((0..self.n).map(|i| self.h_list[i]       * s.5[i]).collect()) - (c_tilde * c) + (G * s.2)).to_affine();
        let t_prime_3: StoredElement = (summation((0..self.n).map(|i| commit_prime_list[i] * s.5[i]).collect()) - (e_prime * c) - (G * s.3)).to_affine();

        let mut t_hat_prime_list = Vec::new();
        for i in 0..self.n {
            if i == 0 {
                t_hat_prime_list.push(((G * s.4[i]) + (self.h_list[0]  * s.5[i]) - (c_hat_list[i] * c)).to_affine());
            } else {
                t_hat_prime_list.push(((G * s.4[i]) + (c_hat_list[i-1] * s.5[i]) - (c_hat_list[i] * c)).to_affine());
            }
        }

        let t_prime = (t_prime_0, t_prime_1, t_prime_2, t_prime_3, t_hat_prime_list);
        // println!("{:#?}", t_prime);
        return t == t_prime
    }
}
