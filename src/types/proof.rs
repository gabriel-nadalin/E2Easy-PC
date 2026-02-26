use p256::ecdsa::VerifyingKey;
use serde::{Deserialize, Serialize};
use crate::{Element, Scalar};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ShuffleProof {
    t: (Element, Element, Element, Element, Vec<Element>),
    s: (Scalar, Scalar, Scalar, Scalar, Vec<Scalar>, Vec<Scalar>),
    c_list: Vec<Element>,
    c_hat_list: Vec<Element>
}

impl ShuffleProof {
    pub fn new(
        t: (Element, Element, Element, Element, Vec<Element>),
        s: (Scalar, Scalar, Scalar, Scalar, Vec<Scalar>, Vec<Scalar>),
        c_list: Vec<Element>,
        c_hat_list: Vec<Element>
    ) -> Self {
        Self {
            t: (
                t.0,
                t.1,
                t.2,
                t.3,
                t.4,
            ),
            s,
            c_list,
            c_hat_list,
        }
    }

    pub fn components(&self) -> (
        (Element, Element, Element, Element, Vec<Element>),
        (Scalar, Scalar, Scalar, Scalar, Vec<Scalar>, Vec<Scalar>),
        Vec<Element>,
        Vec<Element>,
    ) {
        (
            (
                self.t.0,
                self.t.1,
                self.t.2,
                self.t.3,
                self.t.4.clone()
            ),
            self.s.clone(),
            self.c_list.clone(),
            self.c_hat_list.clone()
        )
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ZKPOutput {
    pub verifying_key: VerifyingKey,
    pub shuffle_proof: ShuffleProof,
    pub m_list: Vec<Scalar>,
    pub r_list: Vec<Scalar>
}

impl ZKPOutput {
    pub fn new(verifying_key: VerifyingKey, shuffle_proof: ShuffleProof, m_list: Vec<Scalar>, r_list: Vec<Scalar> ) -> Self {
        Self {
            verifying_key,
            shuffle_proof,
            m_list,
            r_list,
        }
    }
}