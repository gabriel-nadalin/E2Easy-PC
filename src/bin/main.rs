use mixnet_rust::{N, Scalar, io_helpers::read_json, pedersen::Pedersen, shuffler::Shuffler, types::InfoContest, utils::*, verifier::Verifier};
use std::time::Instant;

fn main() {
    println!("N = {:?}", N);

    let info_contest: InfoContest = read_json("./outputs/info_contest.json").unwrap();
    let (h, h_list) = (info_contest.crypto.h, info_contest.crypto.h_list.iter().take(N).cloned().collect::<Vec<_>>());
    let plaintext_list: Vec<_> = (0..N).map(|i| { 
        // Need to improve, but ok for numbers less than 1000 bits (because I removed (mod p))
        let val = (i as u32 + 1) * (i as u32 + 1);
        scalar_from_bytes(&val.to_be_bytes().to_vec())
    }).collect();

    let pedersen = Pedersen::new(&h);
    let shuffler = Shuffler::new(h_list.clone());
    let verifier = Verifier::new(h_list);

    let r_list: Vec<Scalar> = (0..N).map(|_| random_scalar()).collect();
    let commit_list_1 = pedersen.commit_list(&plaintext_list, &r_list);

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
    let result: bool = verifier.check_proof(&proof, &commit_list_1, &commit_list_2,);
    let verify_time = verify_start.elapsed();
    println!("Verify time: {:?}", verify_time);
    println!("result: {result}");

    let combined_r_list: Vec<_> = (0..N).map(|i| r_list[i] + random_list[i]).collect();
    let shuffled_r_list: Vec<_> = (0..N).map(|i| combined_r_list[psi[i]].clone()).collect();
    let shuffled_plaintext_list: Vec<_> = (0..N).map(|i| plaintext_list[psi[i]].clone()).collect();

    let commits_start = Instant::now();
    let result_commits: bool = pedersen.verify_list(&shuffled_plaintext_list, &shuffled_r_list, &commit_list_2);
    let commits_time = commits_start.elapsed();
    println!("Commits verify time: {:?}", commits_time);
    println!("result_commits: {result_commits}");
}
