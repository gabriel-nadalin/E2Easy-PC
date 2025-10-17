use std::sync::Arc;

use chrono::Utc;

use sha2::{Digest, Sha256};
use ed25519_dalek::Signature;
use crate::{groups::{Element, Group}, keys::{self, EncryptionKeys, SignatureKeys}, types::*, Ciphertext};

pub struct E2Easy<G: Group> {
    pub group: Arc<G>,
    pub enc_keys: EncryptionKeys<G>,
    sig_keys: SignatureKeys,
    pub vote_table: VoteTable<G>,
    votes: Vec<Vote>,
    nonces: Vec<G::Scalar>,
    enc_votes: Vec<Ciphertext<G>>,
    timestamp: String,
    tracking_code: TrackingCode,
    prev_tracking_code: TrackingCode,
    
    // shuffler: Shuffler,
}

impl<G: Group> E2Easy<G> {
    // seria possivel combinar setup() e start() em new()?
    pub fn new(group: Arc<G>) -> Self {
        let (enc_keys, sig_keys) = keys::keygen(group.clone());
        Self {
            group,
            enc_keys,
            sig_keys,
            vote_table: VoteTable::new(),
            votes: Vec::new(),
            nonces: Vec::new(),
            enc_votes: Vec::new(),
            timestamp: "".to_string(),
            tracking_code: TrackingCode(Vec::new()),
            prev_tracking_code: TrackingCode(Sha256::digest(b"start").to_vec()),
        }
    }

    pub fn setup() {
        todo!()
    }

    pub fn start() {
        todo!()
    }

    pub fn vote(&mut self, votes: Vec<Vote>) -> (TrackingCode, String) {
        self.timestamp = Utc::now().to_rfc3339();

        let mut to_hash = self.prev_tracking_code.0.clone();
        to_hash.extend_from_slice(self.timestamp.as_bytes());

        for vote in votes {
            let r = self.group.random_scalar();
            let encoded_vote = self.group.deserialize_to_element(vote.to_bytes());
            let encrypted_vote = self.enc_keys.encrypt(&encoded_vote, &r);

            to_hash.extend_from_slice(&[encrypted_vote.0.serialize(), encrypted_vote.1.serialize()].concat());

            self.votes.push(vote);
            self.nonces.push(r);
            self.enc_votes.push(encrypted_vote);
        }
        
        self.tracking_code = TrackingCode(Sha256::digest(to_hash).to_vec());
        (self.tracking_code.clone(), self.timestamp.clone())
    }

    pub fn challenge(&mut self) -> (TrackingCode, Vec<Vote>, Vec<G::Scalar>) {
        let output = (self.prev_tracking_code.clone(), self.votes.clone(), self.nonces.clone());

        self.tracking_code = TrackingCode(Vec::new());
        self.timestamp = "".to_string();
        self.votes = Vec::new();
        self.nonces = Vec::new();
        self.enc_votes = Vec::new();

        output
    }

    pub fn cast(&mut self) -> Signature {
        let signature = self.sig_keys.sign(self.tracking_code.0.clone());
        for enc_vote in self.enc_votes.clone() {
            let entry = VoteTableEntry {
                tracking_code: self.tracking_code.clone(),
                enc_vote,
                time: self.timestamp.clone()
            };
            self.vote_table.add_entry(entry);

        }
        self.prev_tracking_code = self.tracking_code.clone();

        self.tracking_code = TrackingCode(Vec::new());
        self.timestamp = "".to_string();
        self.votes = Vec::new();
        self.nonces = Vec::new();
        self.enc_votes = Vec::new();
        
        signature
    }

    pub fn tally() -> (Proofs, VoteTable<G>, VoteOutput) {
        todo!()
    }

    pub fn finish() {
        todo!()
    }
}