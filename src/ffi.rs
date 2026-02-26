use safer_ffi::prelude::*;
use crate::e2easy::E2Easy;
use crate::io_helpers::read_json;
use crate::types::{ballot::*, config::*};

// Simple result wrapper
#[derive_ReprC]
#[repr(C)]
pub struct JsonResult {
    pub success: bool,
    pub data: char_p::Box,      // JSON string or error message
}

#[ffi_export]
fn e2easy_new() -> Option<repr_c::Box<E2Easy>> {
    let info_contest: InfoContest = read_json("./outputs/info_contest.json").ok()?;
    let (h, h_list) = (info_contest.crypto.h, info_contest.crypto.h_list);
    Some(Box::new(E2Easy::new(&h, h_list.to_vec())).into())
}

#[ffi_export]
fn e2easy_free(it: repr_c::Box<E2Easy>) {
    drop(it)
}

#[ffi_export]
fn e2easy_vote(
    handle: &mut repr_c::Box<E2Easy>,
    votes_json: char_p::Ref<'_>
) -> JsonResult {
    let votes_str = votes_json.to_str();
    let votes: Vec<Vote> = match serde_json::from_str(votes_str) {
        Ok(v) => v,
        Err(e) => return JsonResult {
            success: false,
            data: format!("Invalid votes JSON: {}", e).try_into().unwrap(),
        }
    };
    
    let (tracking_code, timestamp) = handle.vote(votes);
    
    let result = serde_json::json!({
        "tracking_code": tracking_code,
        "timestamp": timestamp
    });
    
    JsonResult {
        success: true,
        data: result.to_string().try_into().unwrap(),
    }
}

#[ffi_export]
fn e2easy_challenge(
    handle: &mut repr_c::Box<E2Easy>,
) -> JsonResult {
    let (tracking_code, committed_votes, nonce_seed) = handle.challenge();
    
    let result = serde_json::json!({
        "tracking_code": tracking_code,
        "committed_votes": committed_votes,
        "nonce_seed": nonce_seed
    });
    
    JsonResult {
        success: true,
        data: result.to_string().try_into().unwrap(),
    }
}

#[ffi_export]
fn e2easy_cast(
    handle: &mut repr_c::Box<E2Easy>,
) -> JsonResult {
    let signature = handle.cast();
    
    let result = serde_json::json!({
        "signature": signature
    });
    
    JsonResult {
        success: true,
        data: result.to_string().try_into().unwrap(),
    }
}

#[ffi_export]
fn e2easy_tally(
    handle: &mut repr_c::Box<E2Easy>,
) -> JsonResult {
    let (rdv_prime, rdcv, rdcv_prime, zkp) = handle.tally();
    
    // Return as separate JSON objects for artifact separation
    let result = serde_json::json!({
        "rdv_prime": rdv_prime,
        "rdcv": rdcv,
        "rdcv_prime": rdcv_prime,
        "zkp": zkp
    });
    
    JsonResult {
        success: true,
        data: result.to_string().try_into().unwrap(),
    }
}

#[ffi_export]
fn json_result_free(result: JsonResult) {
    drop(result)
}