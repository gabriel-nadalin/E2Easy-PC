use mixnet_rust::{groups::bigint_mod::{BigIntModGroup, BigIntModElement}, groups::traits::Scalar, keys, shuffler::Shuffler, pedersen::Pedersen, utils::*, verifier::Verifier, N};
use mixnet_rust::groups::traits::Group;
use std::time::Instant;

fn main() {
    let (p, q, g) = get_group_params();
    let group = BigIntModGroup::new(p, q, g);
    println!("N = {:?}", N);

    // let (enc_keys, _sig_keys) = keys::keygen(group.clone());

    let h_list: Vec<BigIntModElement> = (0..N).map(|_| group.random_element()).collect();

    let shuffler = Shuffler::new(group.clone(), h_list.to_vec());

    let plaintext_list: Vec<_> = (0..N).map(|i| { 
        // Need to improve, but ok for numbers less than 1000 bits (because I removed (mod p))
        let val = (i as u32 + 1) * (i as u32 + 1);
        group.element_from_bytes(&val.to_be_bytes().to_vec())
    }).collect();

    let pedersen = Pedersen::new(group.clone());
    let (commit_list_1, r_list) = pedersen.commit_list(&plaintext_list);
    // let res = pedersen.verify_list(&plaintext_list, &r_list, &commit_list_1);
    // println!("Test: {:?}", res);

    // Do I already verify the commitments are right? I don't see why

    // let ciphertext_list_1: Vec<_> = (0..N).map(|i| {
    //     let r = group.random_scalar();
    //     enc_keys.encrypt(&plaintext_list[i], &r)
    // }).collect();
    
    // println!("plaintext: {:?}", plaintext_list);
    // println!("ciphertext: {:?}", ciphertext_list_1);

    let mixing_start = Instant::now();
    let (commit_list_2, random_list, psi) = shuffler.gen_shuffle(&commit_list_1);
    let proof = shuffler.gen_proof(
        &commit_list_1,
        &commit_list_2,
        &random_list,
        &psi
    );
    let mixing_time = mixing_start.elapsed();
    println!("Mixing time: {:?}", mixing_time);
    // println!("shuffled: {:?}", ciphertext_list_2);
    // println!("proof: {:?}", proof);

    let verify_start = Instant::now();
    let verifier = Verifier::new(group.clone(), h_list);
    let result: bool = verifier.check_proof(&proof, &commit_list_1, &commit_list_2,);
    let verify_time = verify_start.elapsed();
    println!("Verify time: {:?}", verify_time);
    println!("result: {result}");

    let combined_r_list: Vec<_> = (0..N).map(|i| r_list[i].add(&random_list[i])).collect();
    let shuffled_r_list: Vec<_> = (0..N).map(|i| combined_r_list[psi[i]].clone()).collect();
    let shuffled_plaintext_list: Vec<_> = (0..N).map(|i| plaintext_list[psi[i]].clone()).collect();

    let commits_start = Instant::now();
    let result_commits: bool = pedersen.verify_list(&shuffled_plaintext_list, &shuffled_r_list, &commit_list_2);
    let commits_time = commits_start.elapsed();
    println!("Commits verify time: {:?}", commits_time);
    println!("result_commits: {result_commits}");

    // let decrypted_list: [BigIntModElement; N] = core::array::from_fn(|i| enc_keys.sk.decrypt(&ciphertext_list_2[i].clone()));
    // println!("shuffled & decrypted: {:?}", decrypted_list);
}
