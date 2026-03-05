use serde::{Deserialize, Serialize};
use crate::{Element, Scalar, utils::scalar_from_bytes};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
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

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let n = bytes.len();
        let contest = bytes[n-2];
        let choice = bytes[n-1];
        Self::new(contest, choice)
    }
    
    pub fn to_scalar(&self) -> Scalar {
        let bytes = self.to_bytes();
        scalar_from_bytes(&bytes)
    }

    pub fn from_scalar(scalar: Scalar) -> Self {
        let bytes = scalar.to_bytes();
        Self::from_bytes(&bytes)
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