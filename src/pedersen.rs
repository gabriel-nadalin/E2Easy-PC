use crate::{groups::traits::{Element, Group}};//, keys::PublicKey
use std::sync::Arc;

pub struct Pedersen<G: Group> {
    group: Arc<G>,
    h: G::Element,
//    pk: G::Scalar,
//    sk: G::Scalar,
//    pk: PublicKey<G>,
}

impl<G: Group> Pedersen<G> {
    pub fn new(group: Arc<G>) -> Self {
        let h = group.random_element();
        Self {
            group,
            h,
        }
    }

    pub fn commit (&self, plaintext: &G::Scalar) -> (G::Element, G::Scalar) {
        let r = self.group.random_scalar();
        let c = self.group.mul_generator(&r).add(&self.h.mul_scalar(&plaintext));
        return (c, r)
    }

    pub fn commit_list (&self, plaintext_list: &Vec<G::Scalar>) -> (Vec<G::Element>, Vec<G::Scalar>) {
        let mut commit_list = Vec::new();
        let mut r_list = Vec::new();
        let n = plaintext_list.len();

        for i in 0..n {
            let (c, r) = self.commit(&plaintext_list[i]);
            commit_list.push(c);
            r_list.push(r);
        }

        return (commit_list, r_list)
    }

    pub fn verify (&self, plaintext: &G::Scalar, r: &G::Scalar, commit: &G::Element) -> bool {
        let commit_prime = self.group.mul_generator(&r).add(&self.h.mul_scalar(&plaintext));
        return *commit == commit_prime
    }

    pub fn verify_list (&self, plaintext_list: &Vec<G::Scalar>, r_list: &Vec<G::Scalar>, commit_list: &Vec<G::Element>) -> bool {
        let n = r_list.len();
        let mut result: bool = true;

        for i in 0..n {
            result = self.verify(&plaintext_list[i], &r_list[i], &commit_list[i]);
            if result == false {
                break;
            }
        }

        return result
    }
}
