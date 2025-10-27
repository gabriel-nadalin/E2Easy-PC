use mixnet_rust::{Number, types::Ciphertext, groups::bigint_mod::{BigIntModGroup, BigIntModElement}, keys, shuffler::Shuffler, utils::*, verifier::Verifier, N};
use mixnet_rust::groups::traits::Group;
use std::time::Instant;

fn main() {
    let (p, q, g) = get_group_params();
    let group = BigIntModGroup::new(p, q, g);

    println!("N = {:?}", N);

    let (enc_keys, _sig_keys) = keys::keygen(group.clone());

    let h_list: Vec<BigIntModElement> = (0..N).map(|_| group.random_element()).collect();

    let shuffler = Shuffler::new(group.clone(), h_list.to_vec(), &enc_keys.pk);

    let plaintext_list: Vec<_> = (0..N).map(|i| { 
        // Need to improve, but ok for numbers less than 1000 bits (because I removed (mod p))
        let val = (i as u32 + 1) * (i as u32 + 1);
        group.element_from_bytes(&val.to_be_bytes().to_vec())
    }).collect();

    let ciphertext_list_1: Vec<_> = (0..N).map(|i| {
        let r = group.random_scalar();
        enc_keys.encrypt(&plaintext_list[i], &r)
    }).collect();
    
    // println!("plaintext: {:?}", plaintext_list);
    // println!("ciphertext: {:?}", ciphertext_list_1);

    let mixing_start = Instant::now();
    let (ciphertext_list_2, random_list, psi) = shuffler.gen_shuffle(&ciphertext_list_1);
    let proof = shuffler.gen_proof(
        &ciphertext_list_1,
        &ciphertext_list_2,
        &random_list,
        &psi
    );
    let mixing_time = mixing_start.elapsed();
    println!("Mixing time: {:?}", mixing_time);
    // println!("shuffled: {:?}", ciphertext_list_2);
    // println!("proof: {:?}", proof);

    let verify_start = Instant::now();
    let verifier = Verifier::new(group.clone(), h_list);
    let result = verifier.check_proof(&proof, &ciphertext_list_1, &ciphertext_list_2, &enc_keys.pk);
    let verify_time = verify_start.elapsed();
    println!("Mixing time: {:?}", verify_time);
    println!("result: {result}");

    let decrypted_list: [Number; N] = core::array::from_fn(|i| enc_keys.sk.decrypt(&ciphertext_list_2[i].clone()).value.retrieve());
    // println!("shuffled & decrypted: {:?}", decrypted_list);
}
