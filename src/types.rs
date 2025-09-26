use std::collections::HashMap;

pub struct Vote {
    pub contest: u8,
    pub choice: u8,
}

impl Vote {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.contest, self.choice]
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TrackingCode (pub Vec<u8>);

pub struct Signature (pub Vec<u8>);

pub struct Proofs (Vec<u8>);

pub struct Hash (Vec<u8>);

pub struct Nonce (Vec<u8>);

pub struct VoteTableEntry {
    pub tracking_code: TrackingCode,
    pub vote: Vote,
    pub enc_vote: Vec<u8>,
    pub time: String,
}

pub struct VoteTable {
    entries: Vec<VoteTableEntry>,
}

impl VoteTable {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_vote(&mut self, entry: VoteTableEntry) {
        self.entries.push(entry);
    }

    pub fn last_tc(&self) -> &TrackingCode {
        &self.entries.last().unwrap().tracking_code
    }
}

pub struct VoteOutput {
    value: String           // ?
}