use std::fmt;

use crate::{Scalar, Element};


/*
#[derive(Clone, PartialEq)]
pub struct Ciphertext<G: Group> {
    c1: G::Element,
    c2: G::Element,
}

impl<G: Group> Ciphertext<G> {
    pub fn new(c1: G::Element, c2: G::Element) -> Self {
        Self {
            c1,
            c2,
        }
    }

    pub fn components(&self) -> (&G::Element, &G::Element) {
        (&self.c1, &self.c2)
    }

    pub fn c1(&self) -> &G::Element {
        &self.c1
    }

    pub fn c2(&self) -> &G::Element {
        &self.c2
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [self.c1.to_bytes(), self.c2.to_bytes()].concat()
    }
}

impl<G: Group> fmt::Debug for Ciphertext<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?},{:?})", self.c1, self.c2)
    }
}
*/

#[derive(Clone, PartialEq)]
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
            t,
            s,
            c_list,
            c_hat_list
        }
    }

    pub fn components(&self) -> (
        &(Element, Element, Element, Element, Vec<Element>),
        &(Scalar, Scalar, Scalar, Scalar, Vec<Scalar>, Vec<Scalar>),
        &Vec<Element>,
        &Vec<Element>,
    ) {
        (&self.t, &self.s, &self.c_list, &self.c_hat_list)
    }
}

impl fmt::Debug for ShuffleProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("({:?},{:?},{:?},{:?})", self.t, self.s, self.c_list, self.c_hat_list).replace(" ", ""))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vote {
    pub contest: u8,
    pub choice: u8,
}

impl Vote {
    pub fn new(contest: u8, choice: u8) -> Self {
        Self {
            contest,
            choice
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [self.contest.to_be_bytes().as_slice(), self.choice.to_be_bytes().as_slice()].concat()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ballot {
    pub tracking_code: TrackingCode,
    pub enc_votes: Vec<Element>,
    pub timestamp: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VoteTable {
    entries: Vec<Ballot>,
}

impl VoteTable {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: Ballot) {
        self.entries.push(entry);
    }

    pub fn last_tc(&self) -> &TrackingCode {
        &self.entries.last().unwrap().tracking_code
    }

    pub fn votes(&self) -> Vec<Element> {
        self.entries.iter().flat_map(|entry| entry.enc_votes.clone()).collect()
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TrackingCode (pub Vec<u8>);

impl fmt::Debug for TrackingCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.clone()))
    }
}

pub struct Signature (pub Vec<u8>);

pub struct Proofs (Vec<u8>);

pub struct Hash (Vec<u8>);

pub struct Nonce (Vec<u8>);

pub struct VoteOutput {
    value: String           // ?
}
