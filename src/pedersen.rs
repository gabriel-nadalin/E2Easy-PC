use crate::{Scalar, Element, G, utils::*};

pub struct Pedersen {
    h: Element,
}

impl Pedersen {
    pub fn new() -> Self {
        let h = random_element();
        Self {
            h,
        }
    }

    pub fn commit (&self, plaintext: &Scalar) -> (Element, Scalar) {
        let r = random_scalar();
        let c = (G * r) + (self.h * plaintext);
        return (c, r)
    }

    pub fn commit_list (&self, plaintext_list: &Vec<Scalar>) -> (Vec<Element>, Vec<Scalar>) {
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

    pub fn verify (&self, plaintext: &Scalar, r: &Scalar, commit: &Element) -> bool {
        let commit_prime = (G * r) + (self.h * plaintext);
        return *commit == commit_prime
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

        return result
    }
}
