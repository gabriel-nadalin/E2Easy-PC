use hex::ToHex;
use p256::{FieldBytes, ProjectivePoint, U256, elliptic_curve::{Field, PrimeField, ops::Reduce}};
use serde::Serialize;
use crate::{Scalar, Element, G};
use sha2::{Digest, Sha256};
use rand_core::OsRng;

pub fn random_element() -> Element {
    let randomizer: Scalar = Scalar::random(&mut OsRng);
    return (G * randomizer).into()
}

pub fn random_scalar() -> Scalar {
    return Scalar::random(&mut OsRng)
}

pub fn summation (list: Vec<ProjectivePoint>) -> ProjectivePoint {
    let mut sum: ProjectivePoint = ProjectivePoint::IDENTITY;
    let n = list.len();
    for i in 0..n {
        sum += list[i];
    }
    return sum
}


// strict: only accepts canonical 32-byte scalar encoding.
pub fn scalar_from_bytes_strict(bytes: &[u8]) -> Option<Scalar> {
    if bytes.len() != 32 {
        return None;
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);

    Scalar::from_repr(FieldBytes::from(arr)).into()
}

pub fn derive_nonces(seed: &Scalar, count: usize) -> Vec<Scalar> {
    let mut nonces = Vec::with_capacity(count);
    for i in 0..count {
        let to_hash = (seed, i);
        nonces.push(hash2scalar(&to_hash));
    }
    nonces
}

/// hashes a serializable object into a hex string
pub fn hash2str<T: Serialize + ?Sized>(obj: &T) -> String {
    Sha256::digest(serde_json_canonicalizer::to_vec(&obj).unwrap()).encode_hex_upper()
}

/// hashes a serializable object into a scalar
pub fn hash2scalar<T: Serialize + ?Sized>(obj: &T) -> Scalar {
    let digest = Sha256::digest(serde_json_canonicalizer::to_vec(&obj).unwrap());
    <Scalar as Reduce<U256>>::reduce_bytes(&FieldBytes::from(digest))
}