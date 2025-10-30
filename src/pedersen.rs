use crate::{groups::traits::{Element, Group, Scalar}};//, keys::PublicKey
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

    pub fn commit (&self, plaintext: &G::Element) -> (G::Element, G::Scalar) {
        let r = self.group.random_scalar();
        let c = self.group.mul_generator(&r).add(&self.h.mul_scalar(&plaintext.to_scalar()));
        return (c, r)
    }

    pub fn commit_list (&self, h_list: &Vec<G::Element>) -> (Vec<G::Element>, Vec<G::Scalar>) {
        let mut commit_list = Vec::new();
        let mut r_list = Vec::new();
        let n = h_list.len();

        for i in 0..n {
            let (c, r) = self.commit(&h_list[i]);
            commit_list.push(c);
            r_list.push(r);
        }

        return (commit_list, r_list)
    }

    pub fn verify (&self, commit: &G::Element, r: &G::Scalar, plaintext: &G::Element) -> bool {
        let commit_prime = self.group.mul_generator(&r).add(&self.h.mul_scalar(&plaintext.to_scalar()));
        return *commit == commit_prime
    }

    pub fn verify_list (&self, commit_list: &Vec<G::Element>, r_list: &Vec<G::Scalar>, plaintext_list: &Vec<G::Element>) -> bool {
        let n = r_list.len();
        let mut result: bool = true;

        for i in 0..n {
            result = self.verify(&commit_list[i], &r_list[i], &plaintext_list[i]);
            if result == false {
                break;
            }
        }

        return result
    }
}
