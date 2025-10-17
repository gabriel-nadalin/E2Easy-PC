use std::fmt;

use crate::{groups::Group, Ciphertext};


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

#[derive(Clone, PartialEq, Debug)]
pub struct VoteTableEntry<G: Group> {
    pub tracking_code: TrackingCode,
    // pub votes: Vec<Vote>,
    // pub nonces: Vec<Nonce>,
    pub enc_vote: Ciphertext<G>,
    pub time: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VoteTable<G: Group> {
    entries: Vec<VoteTableEntry<G>>,
}

impl<G: Group> VoteTable<G> {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: VoteTableEntry<G>) {
        self.entries.push(entry);
    }

    pub fn last_tc(&self) -> &TrackingCode {
        &self.entries.last().unwrap().tracking_code
    }

    pub fn votes(&self) -> Vec<Ciphertext<G>> {
        self.entries.iter().map(|entry| entry.enc_vote.clone()).collect()
    }
}

pub struct VoteOutput {
    value: String           // ?
}