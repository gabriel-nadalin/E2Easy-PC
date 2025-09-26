use crypto_bigint::{Encoding, RandomMod, rand_core::OsRng};
use num_primes::{BigUint, Generator};
use std::hash::{Hash, Hasher, DefaultHasher};
use core::array::from_fn;

use crate::{N, Ciphertext, Number, NumberNZ, ModNumber, ModNumberParams, SIZE};

pub fn biguint_to_uint(n: &BigUint) -> Number {
    let n_bytes = n.to_bytes_be();
    let mut n_array = [0; Number::BYTES];
    n_array[Number::BYTES - n_bytes.len()..].copy_from_slice(&n_bytes);
    Number::from_be_bytes(n_array)
}

// To make compatible with hash
pub fn modnumber_to_number (list: [ModNumber; N]) -> [Number; N] {
    from_fn(|i| list[i].retrieve())
}

// To make compatible with hash
pub fn ciphertext_to_number (list: [Ciphertext; N]) -> [(Number, Number); N] {
    from_fn(|i| (list[i].0.retrieve(), list[i].1.retrieve()))
}

pub fn get_random(n: &NumberNZ) -> Option<Number> {
//    return Some(Number::from_be_hex("5FA555BBCEDABC7208686E16B8019228A3089E8E74ECE6A915FA01DFA0A02B09"))
    let a = Number::random_mod(&mut OsRng, &n);
//    println!("rand = {:?}", a);
    return Some(a)
}

pub fn get_generator(n: &ModNumberParams) -> Option<ModNumber> {
    let mut temp_g: Number;
    loop {
        temp_g = get_random(n.modulus().as_nz_ref()).unwrap();
        if temp_g > Number::ONE {
            break;
        }
    }
    let g = ModNumber::new(&temp_g, *n); 
    return Some(g.square())
}

pub fn safe_prime() -> Option<(ModNumberParams, NumberNZ)> {
    let temp_p: BigUint = Generator::safe_prime(SIZE);
    let temp_q: BigUint = (&temp_p - 1u32) >> 1;
    let p = ModNumberParams::new_vartime(biguint_to_uint(&temp_p).to_odd().unwrap());
    let q = biguint_to_uint(&temp_q).to_nz().unwrap();
    return Some((p,q))
}

pub fn hash<T: Hash>(t: T) -> Number {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    Number::from_u64(s.finish())
}

pub fn prod(list: [ModNumber; N], modulo: ModNumberParams) -> ModNumber {
    let mut result = ModNumber::new(&Number::ONE, modulo);
    for i in 0..N {
        result = result.mul(&list[i]);
    }
//    println!("result = {:?}", result.retrieve());
    return result
}
