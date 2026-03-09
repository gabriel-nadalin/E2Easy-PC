use serde::{Deserialize, Serialize};
use crate::{Element};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CryptoParams {
    pub h: Element,          // the base generator for commitments
    pub h_list: Vec<Element> // per-contest generators
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ContestInfo {
    pub contest_id: u32,
    pub name: String,
    pub options: Vec<OptionInfo>,       // total options available
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct OptionInfo {
    pub option_id: u32,
    pub name: String
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ElectionConfig {
    pub crypto: CryptoParams,       
    pub contests: Vec<ContestInfo>, 
}
