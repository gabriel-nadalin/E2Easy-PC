use std::fmt;

use crate::groups::traits::{Element, Group};


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

#[derive(Clone, PartialEq)]
pub struct ShuffleProof<G: Group> {
    t: (G::Element, G::Element, G::Element, G::Element, Vec<G::Element>),
    s: (G::Scalar, G::Scalar, G::Scalar, G::Scalar, Vec<G::Scalar>, Vec<G::Scalar>),
    c_list: Vec<G::Element>,
    c_hat_list: Vec<G::Element>
}

impl<G: Group> ShuffleProof<G> {
    pub fn new(
        t: (G::Element, G::Element, G::Element, G::Element, Vec<G::Element>),
        s: (G::Scalar, G::Scalar, G::Scalar, G::Scalar, Vec<G::Scalar>, Vec<G::Scalar>),
        c_list: Vec<G::Element>,
        c_hat_list: Vec<G::Element>
    ) -> Self {
        Self {
            t,
            s,
            c_list,
            c_hat_list
        }
    }

    pub fn components(&self) -> (
        &(G::Element, G::Element, G::Element, G::Element, Vec<G::Element>),
        &(G::Scalar, G::Scalar, G::Scalar, G::Scalar, Vec<G::Scalar>, Vec<G::Scalar>),
        &Vec<G::Element>,
        &Vec<G::Element>,
    ) {
        (&self.t, &self.s, &self.c_list, &self.c_hat_list)
    }
}

impl<G: Group> fmt::Debug for ShuffleProof<G> {
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
pub struct Ballot<G: Group> {
    pub tracking_code: TrackingCode,
    pub enc_votes: Vec<Ciphertext<G>>,
    pub timestamp: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VoteTable<G: Group> {
    entries: Vec<Ballot<G>>,
}

impl<G: Group> VoteTable<G> {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: Ballot<G>) {
        self.entries.push(entry);
    }

    pub fn last_tc(&self) -> &TrackingCode {
        &self.entries.last().unwrap().tracking_code
    }

    pub fn votes(&self) -> Vec<Ciphertext<G>> {
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
