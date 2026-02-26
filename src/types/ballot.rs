use serde::{Deserialize, Deserializer, Serialize};
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
    pub scalar_votes: Vec<Scalar>,
    pub committed_votes: Vec<Element>,
    pub nonce_seed: Scalar,
    pub timestamp: String,
    pub tracking_code: TrackingCode,
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CommittedBallot {
    pub tracking_code: TrackingCode,
    pub committed_votes: Vec<Element>,
    pub timestamp: String,
}

impl CommittedBallot {
    pub fn new(tracking_code: TrackingCode, committed_votes: Vec<Element>, timestamp: String) -> Self {
        Self {
            tracking_code,
            committed_votes,
            timestamp,
        }
    }

    pub fn votes(&self) -> Vec<Element> {
        self.committed_votes.clone()
    }
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct TrackingCode (pub Vec<u8>);

impl Serialize for TrackingCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&hex::encode_upper(self.0.clone()))
    }
}

impl<'de> Deserialize<'de> for TrackingCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_string = String::deserialize(deserializer)?;
        let bytes = hex::decode(&hex_string)
            .map_err(serde::de::Error::custom)?;
        Ok(TrackingCode(bytes))
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
    tail: TrackingCode,
    entries: Vec<CommittedBallot>,
    head: Option<TrackingCode>,
}

impl RDCV {
    pub fn new(tail: TrackingCode) -> Self {
        Self {
            tail,
            entries: Vec::new(),
            head: None
        }
    }

    pub fn set_head(&mut self, head: TrackingCode) {
        self.head = Some(head);
    }

    pub fn add_entry(&mut self, entry: CommittedBallot) {
        self.entries.push(entry);
    }

    pub fn votes(&self) -> Vec<Element> {
        self.entries.iter().flat_map(|entry| entry.votes()).collect()
    }

    pub fn tail(&self) -> &TrackingCode { &self.tail }

    pub fn entries(&self) -> &[CommittedBallot] { &self.entries }

    pub fn head(&self) -> &Option<TrackingCode> { &self.head }
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