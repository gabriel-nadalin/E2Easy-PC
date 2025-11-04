use p256::{elliptic_curve::PrimeField, FieldBytes, elliptic_curve::Field};
use crate::{Scalar, Element, G, SIZE};
use sha2::{Digest, Sha256};
use rand_core::OsRng;

pub fn random_element() -> Element {
    let randomizer: Scalar = Scalar::random(&mut OsRng);
    return G * randomizer
}

pub fn random_scalar() -> Scalar {
    return Scalar::random(&mut OsRng)
}

pub fn summation (list: Vec<Element>) -> Element {
    let mut sum: Element = Element::IDENTITY;
    let n = list.len();
    for i in 0..n {
        sum += list[i];
    }
    return sum
}

pub fn scalar_from_bytes (bytes: &[u8]) -> Scalar {
    let mut arr = [0; SIZE/8];
    for (i, val) in bytes.into_iter().take(SIZE/8).rev().enumerate() {
        arr[SIZE/8-1 - i] = *val;
    }
    return Scalar::from_repr(*FieldBytes::from_slice(&arr)).unwrap()
}

pub fn derive_nonces (seed: &[u8], count: usize) -> Vec<Scalar> {
    let mut nonces = Vec::with_capacity(count);
    for i in 0..count {
        let mut hasher = Sha256::new();
        hasher.update(seed);
        hasher.update(i.to_be_bytes());
        let hash = hasher.finalize();
        nonces.push(scalar_from_bytes(&hash));
    }
    nonces
}
