use bincode::{Encode, Decode};


#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct Vote {
    pub contest: u8,
    pub choice: u8,
}

impl Vote {
    pub fn to_bytes(&self) -> Vec<u8> {
        [self.contest.to_be_bytes().as_slice(), self.choice.to_be_bytes().as_slice()].concat()
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct TrackingCode (pub Vec<u8>);

pub struct Signature (pub Vec<u8>);

pub struct Proofs (Vec<u8>);

pub struct Hash (Vec<u8>);

pub struct Nonce (Vec<u8>);

#[derive(Clone, PartialEq, Debug)]
pub struct VoteTableEntry {
    pub tracking_code: TrackingCode,
    pub votes: Vec<Vote>,
    // pub nonces: Vec<Nonce>,
    // pub enc_vote: Vec<u8>,
    pub time: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VoteTable {
    entries: Vec<VoteTableEntry>,
}

impl VoteTable {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: VoteTableEntry) {
        self.entries.push(entry);
    }

    pub fn last_tc(&self) -> &TrackingCode {
        &self.entries.last().unwrap().tracking_code
    }
}

pub struct VoteOutput {
    value: String           // ?
}