use std::sync::Arc;

use chrono::Utc;

use sha2::{Digest, Sha256};
use ed25519_dalek::Signature;
// use crate::{groups::traits::{Group, Scalar}, keys::{self, EncryptionKeys, SignatureKeys}, shuffler::Shuffler, types::*, utils::derive_nonces};
use crate::{Scalar, Element, shuffler::Shuffler, utils::*, types::*};

pub struct E2Easy {
    // pub group: Arc<G>,
    // pub enc_keys: EncryptionKeys<G>,
    // sig_keys: SignatureKeys,
    pub vote_table: VoteTable,
    votes: Vec<Vote>,
    nonce_seed: Scalar,
    enc_votes: Vec<Element>,
    timestamp: String,
    tracking_code: TrackingCode,
    prev_tracking_code: TrackingCode,
    
    // shuffler: Shuffler,
}

impl<G: Group> E2Easy<G> {
    // seria possivel combinar setup() e start() em new()?
    pub fn new() -> Self {
        // let (enc_keys, sig_keys) = keys::keygen(group.clone());
        Self {
            // group: group.clone(),
            // enc_keys,
            // sig_keys,
            vote_table: VoteTable::new(),
            votes: Vec::new(),
            nonce_seed: Scalar::ZERO,
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
        self.nonce_seed = random_scalar();
        let nonces = derive_nonces(&self.nonce_seed.to_bytes(), votes.len());

        self.timestamp = Utc::now().to_rfc3339();

        let mut to_hash = self.prev_tracking_code.0.clone();
        to_hash.extend_from_slice(self.timestamp.as_bytes());

        for (vote, nonce) in votes.iter().zip(nonces) {
            // Continue to adapt from here 
            let encoded_vote = self.group.element_from_bytes(&vote.to_bytes());
            let encrypted_vote = self.enc_keys.encrypt(&encoded_vote, &nonce);

            to_hash.extend_from_slice(&encrypted_vote.to_bytes());

            self.votes.push(vote.clone());
            self.enc_votes.push(encrypted_vote);
        }
        
        self.tracking_code = TrackingCode(Sha256::digest(to_hash).to_vec());
        (self.tracking_code.clone(), self.timestamp.clone())
    }

    pub fn challenge(&mut self) -> (TrackingCode, Vec<Vote>, G::Scalar) {
        let output = (self.prev_tracking_code.clone(), self.votes.clone(), self.nonce_seed.clone());

        self.tracking_code = TrackingCode(Vec::new());
        self.timestamp = "".to_string();
        self.nonce_seed = self.group.zero();
        self.votes = Vec::new();
        self.enc_votes = Vec::new();

        output
    }

    pub fn cast(&mut self) -> Signature {
        let signature = self.sig_keys.sign(self.tracking_code.0.clone());
        let entry = Ballot {
            tracking_code: self.tracking_code.clone(),
            enc_votes: self.enc_votes.clone(),
            timestamp: self.timestamp.clone()
        };
        self.vote_table.add_entry(entry);

        self.prev_tracking_code = self.tracking_code.clone();

        self.tracking_code = TrackingCode(Vec::new());
        self.timestamp = "".to_string();
        self.nonce_seed = self.group.zero();
        self.votes = Vec::new();
        self.enc_votes = Vec::new();
        
        signature
    }

    pub fn tally(&self) -> (Proofs, VoteTable<G>, VoteOutput) {
        let e_list = self.vote_table.votes();
        let h_list: Vec<G::Element> = (0..e_list.len()).map(|_| self.group.random_element()).collect();

        let shuffler = Shuffler::new(self.group.clone(), h_list, &self.enc_keys.pk);

        let (e_prime_list, r_prime_list, psi) = shuffler.gen_shuffle(&e_list);

        let shuffle_proof = shuffler.gen_proof(
            &e_list,
            &e_prime_list,
            &r_prime_list,
            &psi
        );

        println!("{:?}", shuffle_proof);
        todo!()
    }

    pub fn finish() {
        todo!()
    }
}
