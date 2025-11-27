use p256::ecdsa::VerifyingKey;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{Element, Scalar, utils::scalar_from_bytes};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ShuffleProof {
    t: (Element, Element, Element, Element, Vec<Element>),
    s: (Scalar, Scalar, Scalar, Scalar, Vec<Scalar>, Vec<Scalar>),
    c_list: Vec<Element>,
    c_hat_list: Vec<Element>
}

impl ShuffleProof {
    pub fn new(
        t: (Element, Element, Element, Element, Vec<Element>),
        s: (Scalar, Scalar, Scalar, Scalar, Vec<Scalar>, Vec<Scalar>),
        c_list: Vec<Element>,
        c_hat_list: Vec<Element>
    ) -> Self {
        Self {
            t: (
                t.0,
                t.1,
                t.2,
                t.3,
                t.4,
            ),
            s,
            c_list,
            c_hat_list,
        }
    }

    pub fn components(&self) -> (
        (Element, Element, Element, Element, Vec<Element>),
        (Scalar, Scalar, Scalar, Scalar, Vec<Scalar>, Vec<Scalar>),
        Vec<Element>,
        Vec<Element>,
    ) {
        (
            (
                self.t.0,
                self.t.1,
                self.t.2,
                self.t.3,
                self.t.4.clone()
            ),
            self.s.clone(),
            self.c_list.clone(),
            self.c_hat_list.clone()
        )
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CryptoParams {
    pub h: Element,          // the base generator for commitments
    pub h_list: Vec<Element> // per-contest generators
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ContestInfo {
    pub contest_id: u32,
    pub name: String,
    pub num_choices: u16,       // total options available
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct InfoContest {
    pub crypto: CryptoParams,       
    pub contests: Vec<ContestInfo>, 
}


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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RDV {
    entries: Vec<Vote>,
}

impl RDV {
    pub fn new(entries: Vec<Vote>) -> Self {
        Self { entries }
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ZKPOutput {
    pub verifying_key: VerifyingKey,
    pub shuffle_proof: ShuffleProof,
    pub m_list: Vec<Scalar>,
    pub r_list: Vec<Scalar>
}

impl ZKPOutput {
    pub fn new(verifying_key: VerifyingKey, shuffle_proof: ShuffleProof, m_list: Vec<Scalar>, r_list: Vec<Scalar> ) -> Self {
        Self {
            verifying_key,
            shuffle_proof,
            m_list,
            r_list,
        }
    }
}