use chrono::Utc;
use p256::ecdsa::{Signature, SigningKey, signature::SignerMut};
use rand_core::OsRng;
use serde::Serialize;
use crate::{Element, Scalar, pedersen::Pedersen, shuffler::Shuffler, types::*, utils::{derive_nonces, hash, random_scalar}};

struct TempBallot {
    scalar_votes: Vec<Scalar>,
    committed_votes: Vec<Element>,
    nonce_seed: Scalar,
    timestamp: String,
    tracking_code: TrackingCode,
}

impl TempBallot {
    pub fn new(
        scalar_votes: Vec<Scalar>,
        committed_votes: Vec<Element>,
        nonce_seed: Scalar,
        timestamp: String,
        tracking_code: TrackingCode
    ) -> Self {
        Self {
            scalar_votes,
            committed_votes,
            nonce_seed,
            timestamp,
            tracking_code,
        }
    }

    pub fn empty() -> Self {
        Self {
            scalar_votes: Vec::new(),
            committed_votes: Vec::new(),
            nonce_seed: Scalar::ZERO,
            timestamp: "".to_string(),
            tracking_code: TrackingCode(Vec::new()),
        }
    }

    pub fn commit(&self) -> CommittedBallot {
        CommittedBallot::new(self.tracking_code.clone(), self.committed_votes.clone(), self.timestamp.clone())
    }
}

pub struct E2Easy {
    h_list: Vec<Element>,
    pedersen: Pedersen,
    sig_key: SigningKey,
    rdcv: RDCV,
    m_list: Vec<Scalar>,
    r_list: Vec<Scalar>,
    temp_ballot: TempBallot,
    prev_tracking_code: TrackingCode,
}

impl E2Easy {
    // seria possivel combinar setup() e start() em new()?
    pub fn new(h: &Element, h_list: Vec<Element>) -> Self {
        Self {
            h_list,
            pedersen: Pedersen::new(h),
            sig_key: SigningKey::random(&mut OsRng),
            rdcv: RDCV::new(TrackingCode(hash(b"start"))),
            m_list: Vec::new(),
            r_list: Vec::new(),
            temp_ballot: TempBallot::empty(),
            // TODO: criat string de configuracao Q para a cauda do RDCV
            prev_tracking_code: TrackingCode(hash(b"start")),
        }
    }

    pub fn setup() {
        todo!()
    }

    pub fn start() {
        todo!()
    }

    pub fn vote(&mut self, votes: Vec<Vote>) -> (TrackingCode, String) {
        let nonce_seed = random_scalar();
        let nonces = derive_nonces(&nonce_seed.to_bytes(), votes.len());

        let timestamp = Utc::now().to_rfc3339();

        let mut scalar_votes = Vec::new();
        let mut committed_votes = Vec::new();

        
        for (vote, nonce) in votes.iter().zip(nonces) {
            let encoded_vote = vote.to_scalar();
            let committed_vote = self.pedersen.commit(&encoded_vote, &nonce);
            scalar_votes.push(encoded_vote);
            committed_votes.push(committed_vote);
        }
        let to_hash = (&self.prev_tracking_code, &timestamp, &committed_votes);
        
        let tracking_code = TrackingCode(hash(&to_hash));
        
        self.temp_ballot = TempBallot::new(scalar_votes, committed_votes, nonce_seed, timestamp.clone(), tracking_code.clone());
        (tracking_code.clone(), timestamp.clone())
    }

    pub fn challenge(&mut self) -> (TrackingCode, Vec<Element>, Scalar) {
        let output = (self.prev_tracking_code.clone(), self.temp_ballot.committed_votes.clone(), self.temp_ballot.nonce_seed.clone());

        self.temp_ballot = TempBallot::empty();

        output
    }

    pub fn cast(&mut self) -> Signature {
        let signature = self.sig_key.sign(&self.temp_ballot.tracking_code.0);
        let entry = self.temp_ballot.commit();
        self.rdcv.add_entry(entry);
        self.m_list.extend_from_slice(&self.temp_ballot.scalar_votes);
        self.r_list.extend_from_slice(&derive_nonces(&self.temp_ballot.nonce_seed.to_bytes(), self.temp_ballot.scalar_votes.len()));

        self.prev_tracking_code = self.temp_ballot.tracking_code.clone();

        self.temp_ballot = TempBallot::empty();
        
        signature
    }

    pub fn tally(&mut self) -> (RDVPrime, RDCV, RDCVPrime, ZKPOutput) {
        let to_hash = (&self.prev_tracking_code, b"CLOSE");
        let head = TrackingCode(hash(&to_hash));
        self.rdcv.set_head(head);
        
        let c_list = self.rdcv.votes();
        let h_list: Vec<Element> = self.h_list.iter().take(c_list.len()).cloned().collect();

        let shuffler = Shuffler::new(h_list.clone());

        let (c_prime_list, r_prime_list, psi) = shuffler.gen_shuffle(&c_list);

        let s_proof = shuffler.gen_proof(
            &c_list,
            &c_prime_list,
            &r_prime_list,
            &psi
        );

        let combined_r_list: Vec<_> = self.r_list.iter().zip(&r_prime_list).map(|(x,y)| x + y).collect();

        let shuffled_r_list: Vec<_> = psi.iter().map(|&i| combined_r_list[i].clone()).collect();
        let shuffled_m_list: Vec<_> = psi.iter().map(|&i| self.m_list[i].clone()).collect();

        let votes = shuffled_m_list.iter().map(|m| Vote::from_scalar(*m)).collect();
        let rdv_prime = RDVPrime::new(votes);
        let rdcv = self.rdcv.clone();
        let rdcv_prime = RDCVPrime::new(c_prime_list);
        let zkp = ZKPOutput::new(*self.sig_key.verifying_key(), s_proof, shuffled_m_list, shuffled_r_list);

        (rdv_prime, rdcv, rdcv_prime, zkp)
    }

    pub fn finish() {
        todo!()
    }

    pub fn sign<T: Serialize>(&mut self, value: &T) -> Signature {
        let json_bytes = serde_json::to_vec_pretty(value).unwrap();
        self.sig_key.sign(&json_bytes)
    }
}