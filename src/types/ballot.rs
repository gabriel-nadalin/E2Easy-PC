use serde::{Deserialize, Serialize};
use crate::{Element, Scalar, utils::*};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Vote {
    pub choice: u32,
    pub contest: u32,
}

impl Vote {
    pub fn new(choice: u32, contest: u32) -> Self {
        Self {
            choice,
            contest,
        }
    }

    /// converts vote to big-endian byte representation (8 bytes total)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        bytes.extend_from_slice(&self.choice.to_be_bytes());
        bytes.extend_from_slice(&self.contest.to_be_bytes());
        bytes
    }

    /// converts from big-endian bytes
    /// returns None if slice length is invalid
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }

        let choice_bytes: [u8; 4] = bytes[bytes.len()-8..bytes.len()-4]
            .try_into()
            .ok()?;

        let contest_bytes: [u8; 4] = bytes[bytes.len()-4..]
            .try_into()
            .ok()?;
        
        Some(Self::new(
            u32::from_be_bytes(choice_bytes),
            u32::from_be_bytes(contest_bytes)
        ))
    }
    
    /// converts vote to scalar for Pedersen commitment (bijective)
    /// vote is encoded as: contest || choice (8 bytes total)
    /// this is always < p256 order (~32 bytes), so no reduction needed
    pub fn to_scalar(&self) -> Scalar {
        let bytes = self.to_bytes(); // 8 bytes total
        
        // pad to 32 bytes (left-pad with zeros)
        let mut padded = [0u8; 32];
        padded[24..].copy_from_slice(&bytes); // place at end (big-endian)
        
        // this will always succeed since 8 bytes << scalar field size
        scalar_from_bytes_strict(&padded)
            .expect("8-byte vote encoding always fits in scalar field")
    }

    /// reconstructs vote from scalar (bijective inverse)
    /// returns None if scalar doesn't represent a valid vote encoding
    pub fn from_scalar(scalar: &Scalar) -> Option<Self> {
        let bytes = scalar.to_bytes(); // 32 bytes
        
        // extract the last 8 bytes (where we encoded the vote)
        let vote_bytes = &bytes[24..];
        
        Self::from_bytes(vote_bytes)
    }
}

pub struct TempBallot {
    scalar_votes: Vec<Scalar>,
    committed_votes: Vec<Element>,
    nonce_seed: Scalar,
    timestamp: String,
    tracking_code: String,
}

impl TempBallot {
    pub fn new(
        scalar_votes: Vec<Scalar>,
        committed_votes: Vec<Element>,
        nonce_seed: Scalar,
        timestamp: String,
        tracking_code: String
    ) -> Self {
        Self {
            scalar_votes,
            committed_votes,
            nonce_seed,
            timestamp,
            tracking_code,
        }
    }

    pub fn scalar_votes(&self) -> &[Scalar] {
        &self.scalar_votes
    }

    pub fn committed_votes(&self) -> &[Element] {
        &self.committed_votes
    }

    pub fn nonce_seed(&self) -> Scalar {
        self.nonce_seed.clone()
    }

    pub fn timestamp(&self) -> String {
        self.timestamp.clone()
    }

    pub fn tracking_code(&self) -> String {
        self.tracking_code.clone()
    }

    pub fn commit(&self) -> CommittedBallot {
        CommittedBallot::new(self.tracking_code.clone(), self.committed_votes.clone(), self.timestamp.clone())
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CommittedBallot {
    tracking_code: String,
    committed_votes: Vec<Element>,
    timestamp: String,
}

impl CommittedBallot {
    pub fn new(tracking_code: String, committed_votes: Vec<Element>, timestamp: String) -> Self {
        Self {
            tracking_code,
            committed_votes,
            timestamp,
        }
    }

    pub fn components(&self) -> (&String, &[Element], &str) {
        (&self.tracking_code, &self.committed_votes, &self.timestamp)
    }

    pub fn votes(&self) -> Vec<Element> {
        self.committed_votes.clone()
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RDVPrime {
    entries: Vec<Vote>,
}

impl RDVPrime {
    pub fn new(entries: Vec<Vote>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RDCV {
    tail: String,
    entries: Vec<CommittedBallot>,
    head: Option<String>,
}

impl RDCV {
    pub fn new(tail: String) -> Self {
        Self {
            tail,
            entries: Vec::new(),
            head: None
        }
    }

    pub fn set_head(&mut self, head: String) {
        self.head = Some(head);
    }

    pub fn add_entry(&mut self, entry: CommittedBallot) {
        self.entries.push(entry);
    }

    pub fn votes(&self) -> Vec<Element> {
        self.entries.iter().flat_map(|entry| entry.votes()).collect()
    }

    pub fn tail(&self) -> &String { &self.tail }

    pub fn entries(&self) -> &[CommittedBallot] { &self.entries }

    pub fn head(&self) -> &Option<String> { &self.head }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RDCVPrime {
    entries: Vec<Element>
}

impl RDCVPrime {
    pub fn new(entries: Vec<Element>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[Element] { &self.entries }
}