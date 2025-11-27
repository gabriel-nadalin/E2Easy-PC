use crate::{G, Scalar, Element};

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
        ((G * r) + (self.h * plaintext)).into()
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
        *commit == commit_prime.into()
    }

    pub fn verify_list (&self, plaintext_list: &[Scalar], r_list: &[Scalar], commit_list: &[Element]) -> bool {
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
