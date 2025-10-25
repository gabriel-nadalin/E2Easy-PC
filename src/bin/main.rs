use mixnet_rust::{Number, Ciphertext, groups::u32_mod::{U32ModGroup, U32ModElement}, keys, shuffler::Shuffler, utils::*, verifier::Verifier, N};
use mixnet_rust::groups::Group;
use std::time::Instant;

fn main() {
    let (p, q, g) = get_group_params();
    let group = U32ModGroup::new(p, q, g);

    println!("N = {:?}", N);

    let (enc_keys, _sig_keys) = keys::keygen(group.clone());

    let h_list: [U32ModElement; N] = core::array::from_fn(|_| group.random_element());

    let shuffler = Shuffler::new(group.clone(), h_list.clone(), enc_keys.pk.clone());

    let plaintext_list: [U32ModElement; N] = core::array::from_fn(|i| { 
        // Need to improve, but ok for numbers less than 1000 bits (because I removed (mod p))
        let val = (i as u32 + 1) * (i as u32 + 1);
        group.deserialize_to_element(val.to_be_bytes().to_vec())
    });

    let ciphertext_list_1: [Ciphertext<U32ModGroup>; N] = core::array::from_fn(|i| {
        let r = group.random_scalar();
        enc_keys.encrypt(&plaintext_list[i], &r)
    });
    
    // println!("plaintext: {:?}", plaintext_list);
    // println!("ciphertext: {:?}", ciphertext_list_1);

    let mixing_start = Instant::now();
    let (ciphertext_list_2, random_list, psi) = shuffler.gen_shuffle(ciphertext_list_1.clone());
    let proof = shuffler.gen_proof(
        ciphertext_list_1.clone(),
        ciphertext_list_2.clone(),
        random_list.clone(),
        psi
    );
    let mixing_time = mixing_start.elapsed();
    println!("Mixing time: {:?}", mixing_time);
    // println!("shuffled: {:?}", ciphertext_list_2);
    // println!("proof: {:?}", proof);

    let verify_start = Instant::now();
    let verifier = Verifier::new(group.clone(), h_list.clone());
    let result = verifier.check_proof(proof, ciphertext_list_1, ciphertext_list_2.clone(), enc_keys.pk.clone());
    let verify_time = verify_start.elapsed();
    println!("Mixing time: {:?}", verify_time);
    println!("result: {result}");

    let decrypted_list: [Number; N] = core::array::from_fn(|i| enc_keys.sk.decrypt(&ciphertext_list_2[i].clone()).value.retrieve());
    // println!("shuffled & decrypted: {:?}", decrypted_list);
}
