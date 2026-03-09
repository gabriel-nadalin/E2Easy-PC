use chrono::Utc;
use p256::ecdsa::{Signature, SigningKey, signature::SignerMut};
use rand_core::OsRng;
use safer_ffi::derive_ReprC;
use serde::Serialize;
use crate::{
    Element,
    Scalar,
    pedersen::Pedersen,
    shuffler::Shuffler,
    types::*,
    utils::{derive_nonces, hash2str, random_scalar}
};


#[derive_ReprC]
#[repr(opaque)]
pub struct E2Easy {
    h_list: Vec<Element>,
    pedersen: Pedersen,
    sig_key: SigningKey,
    rdcv: RDCV,
    m_list: Vec<Scalar>,
    r_list: Vec<Scalar>,
    temp_ballot: Option<TempBallot>,
    prev_tracking_code: String,
}

impl E2Easy {
    // seria possivel combinar setup() e start() em new()?
    pub fn new(h: &Element, h_list: Vec<Element>) -> Self {
        Self {
            h_list,
            pedersen: Pedersen::new(h),
            sig_key: SigningKey::random(&mut OsRng),
            rdcv: RDCV::new(hash2str("start")),
            m_list: Vec::new(),
            r_list: Vec::new(),
            temp_ballot: None,
            // TODO: criat string de configuracao Q para a cauda do RDCV
            prev_tracking_code: hash2str("start"),
        }
    }

    pub fn start() {
        todo!()
    }

    pub fn vote(&mut self, votes: Vec<Vote>) -> (String, String) {
        let nonce_seed = random_scalar();
        let nonces = derive_nonces(&nonce_seed, votes.len());

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
        
        let tracking_code = hash2str(&to_hash);
        
        self.temp_ballot = Some(TempBallot::new(scalar_votes, committed_votes, nonce_seed, timestamp.clone(), tracking_code.clone()));
        (tracking_code.clone(), timestamp.clone())
    }

    pub fn challenge(&mut self) -> (String, Vec<Element>, Scalar) {
        let ballot = self.temp_ballot.take().expect("No ballot to challenge");
        let output = (self.prev_tracking_code.clone(), ballot.committed_votes().to_vec(), ballot.nonce_seed());

        output
    }

    pub fn cast(&mut self) -> Signature {
        let ballot = self.temp_ballot.take().expect("No ballot to cast");
        let signature = self.sig_key.sign(ballot.tracking_code().as_bytes());
        let entry = ballot.commit();
        self.rdcv.add_entry(entry);
        self.m_list.extend_from_slice(&ballot.scalar_votes());
        self.r_list.extend_from_slice(&derive_nonces(&ballot.nonce_seed(), ballot.scalar_votes().len()));

        self.prev_tracking_code = ballot.tracking_code().clone();
        
        signature
    }

    pub fn tally(&mut self) -> (RDVPrime, RDCV, RDCVPrime, ZKPOutput) {
        let to_hash = (&self.prev_tracking_code, "CLOSE");
        let head = hash2str(&to_hash);
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

        let votes = shuffled_m_list.iter().map(|m| Vote::from_scalar(m).unwrap()).collect();
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
        let json_bytes = serde_json_canonicalizer::to_vec(value).unwrap();
        self.sig_key.sign(&json_bytes)
    }
}