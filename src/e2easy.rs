use std::sync::Arc;
use chrono::Utc;

use sha2::{Digest, Sha256};

use crate::{groups::{Element, Group}, keys::{self, EncryptionKeys, SignatureKeys}, shuffler::Shuffler, types::*};

pub struct E2Easy<G: Group> {
    group: Arc<G>,
    enc_keys: EncryptionKeys<G>,
    sig_keys: SignatureKeys<G>,
    vote_table: VoteTable,
    votes_nonces: Vec<(G::Element, G::Scalar)>,
    tracking_code: TrackingCode,
    prev_tracking_code: TrackingCode,
    
    // shuffler: Shuffler,
}

impl<G: Group> E2Easy<G> {
    // seria possivel combinar setup() e start() em new()?
    pub fn new(group: Arc<G>) -> Self {
        let (enc_keys, sig_keys) = keys::keygen();
        Self {
            group,
            enc_keys,
            sig_keys,
            vote_table: VoteTable::new(),
            votes_nonces: Vec::new(),
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

    // implementacao considerando que os votos ja estao codificados para um elemento do grupo criptografico
    pub fn vote(&mut self, votes: Vec<G::Element>) -> TrackingCode {
        let timestamp = Utc::now().to_rfc3339();

        let mut to_hash = self.prev_tracking_code.0.clone();
        to_hash.append(&mut timestamp.as_bytes().to_vec());
        for vote in votes {
            let r = self.group.random_scalar();
            let (c1, c2) = self.enc_keys.encrypt(&vote, &r);
            let enc_vote = [c1.serialize().as_slice(), c2.serialize().as_slice()].concat();
            self.votes_nonces.push((vote, r));
            
            to_hash.append(&mut enc_vote.clone());
        }
        self.tracking_code = TrackingCode(Sha256::digest(to_hash).to_vec());
        self.tracking_code.clone()
    }

    pub fn challenge(&mut self) -> (TrackingCode, Vec<(G::Element, G::Scalar)>) {
        let output = (self.prev_tracking_code.clone(), self.votes_nonces.clone());
        self.votes_nonces = Vec::new();
        self.tracking_code = TrackingCode(Vec::new());
        output
    }

    pub fn cast(&self) -> Signature {
        let signature = self.sig_keys.sign(&G::Element::deserialize(self.tracking_code.0));
        self.vote_table.add_vote(entry);
        Signature(signature)
    }

    pub fn tally() -> (Proofs, VoteTable, VoteOutput) {
        todo!()
    }

    pub fn finish() {
        todo!()
    }
}