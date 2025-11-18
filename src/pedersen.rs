use crate::{Scalar, Element, G, utils::*};

pub struct Pedersen {
    h: Element,
}

impl Pedersen {
    pub fn new(h: &Element) -> Self {
        Self {
            h: *h,
        }
    }

    pub fn commit (&self, plaintext: &Scalar, r: &Scalar) -> Element {
        (G * r) + (self.h * plaintext)
    }

    pub fn commit_list (&self, plaintext_list: &[Scalar], random_list: &[Scalar]) -> Vec<Element> {
        let mut commit_list = Vec::new();
        let n = plaintext_list.len();

        for i in 0..n {
            let c = self.commit(&plaintext_list[i], &random_list[i]);
            commit_list.push(c);
        }

        commit_list
    }

    pub fn verify (&self, plaintext: &Scalar, r: &Scalar, commit: &Element) -> bool {
        let commit_prime = (G * r) + (self.h * plaintext);
        *commit == commit_prime
    }

    pub fn verify_list (&self, plaintext_list: &Vec<Scalar>, r_list: &Vec<Scalar>, commit_list: &Vec<Element>) -> bool {
        let n = r_list.len();
        let mut result: bool = true;

        for i in 0..n {
            result = self.verify(&plaintext_list[i], &r_list[i], &commit_list[i]);
            if result == false {
                break;
            }
        }

        result
    }
}
