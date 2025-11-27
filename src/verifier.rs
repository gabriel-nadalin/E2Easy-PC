use crate::{G, Scalar, Element, types::ShuffleProof, utils::*};
use rayon::prelude::*;
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

    pub fn check_proof(&self, pi: &ShuffleProof, commit_list: &[Element], commit_prime_list: &[Element]) -> bool {
        assert_eq!(commit_list.len(), self.n, "commit_list must have size {}", self.n);
        assert_eq!(commit_prime_list.len(), self.n, "commit_prime_list must have size {}", self.n);

        let (t, s, c_list, c_hat_list) = pi.components();
        
        let y = (
            commit_list,
            commit_prime_list,
            &c_list,
        );

        let u_list: Vec<Scalar> = (0..self.n)
            .into_par_iter()
            .map(|i| {
                let to_hash = (&y, i);
                scalar_from_bytes(&hash(&to_hash))
            })
            .collect();

        let c_bar = summation(c_list.iter().map(|p| p.to_curve()).collect::<Vec<_>>()) - summation(self.h_list.iter().map(|p| p.to_curve()).collect::<Vec<_>>());
        // Product of u_list
        let u: Scalar = u_list.iter().fold(Scalar::ONE, |acc, x| acc * x);

        let c_hat = c_hat_list[self.n-1].to_curve() - (self.h_list[0] * u);
        let c_tilde = summation((0..self.n).map(|i| c_list[i]      * u_list[i]).collect());
        let e_prime = summation((0..self.n).map(|i| commit_list[i] * u_list[i]).collect());

        // conversao para representacao afim eh necessaria para serializacao canonica
        let y = (
            commit_list,
            commit_prime_list,
            &c_list,
            &c_hat_list,
        );
        let to_hash = (y, &t);
        let c = scalar_from_bytes(&hash(&to_hash));

        let t_prime_0: Element = ((G * s.0) - (c_bar * c)).to_affine();
        let t_prime_1: Element = ((G * s.1) - (c_hat * c)).to_affine();
        let t_prime_2: Element = (summation((0..self.n).map(|i| self.h_list[i]       * s.5[i]).collect()) - (c_tilde * c) + (G * s.2)).to_affine();
        let t_prime_3: Element = (summation((0..self.n).map(|i| commit_prime_list[i] * s.5[i]).collect()) - (e_prime * c) - (G * s.3)).to_affine();

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
