use mixnet_rust::{e2easy::E2Easy, io_helpers::{read_json, write_json_to_file}, pedersen::Pedersen, types::{config::*, ballot::*}, verifier::Verifier};
use std::time::Instant;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdtsc;

#[cfg(target_arch = "x86_64")]
unsafe fn rdtsc() -> u64 {
    unsafe { _rdtsc() }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let voters: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(500);

    let info_contest: InfoContest = read_json("./outputs/info_contest.json").unwrap();
    let available = info_contest.crypto.h_list.len() / info_contest.contests.len();
    let n = (voters).min(available);
    
    let (h, h_list) = (info_contest.crypto.h, info_contest.crypto.h_list.iter().take(n * info_contest.contests.len()).cloned().collect::<Vec<_>>());
    let mut e2easy = E2Easy::new(&h, h_list.clone());
    
    println!("N = {:?}", n);
    
    let mut vote_cycles: u64 = 0;
    let mut cast_cycles: u64 = 0;
    // Simulate voting: all voters cast their votes (no challenges)
    for i in 0..n {
        let mut votes = Vec::new();
        
        // Vote for each contest defined in info_contest
        for (contest_idx, contest) in info_contest.contests.iter().enumerate() {
            let choice = (i % contest.num_choices as usize) as u8;
            votes.push(Vote { 
                contest: contest_idx as u8, 
                choice 
            });
        }
        let cycles_start = unsafe { rdtsc() };
        let (_tracking_code, _timestamp) = e2easy.vote(votes);
        vote_cycles += unsafe { rdtsc() } - cycles_start;

        let cycles_start = unsafe { rdtsc() };
        let _tc_signature = e2easy.cast();
        cast_cycles += unsafe { rdtsc() } - cycles_start;
    }
    println!("Voting cycles: {:?}", vote_cycles);
    println!("Casting cycles: {:?}", cast_cycles);

    // Tally votes with timing
    let mixing_start = Instant::now();
    let cycles_start = unsafe { rdtsc() };

    let (rdv_prime, rdcv, rdcv_prime, zkp_output) = e2easy.tally();

    let mixing_cycles = unsafe { rdtsc() } - cycles_start;
    let mixing_time = mixing_start.elapsed();
    println!("Mixing time: {:?}", mixing_time);
    println!("Mixing cycles: {:?}", mixing_cycles);

    // Verify shuffle proof
    let commit_list = rdcv.votes();
    let commit_prime_list = rdcv_prime.entries();
    
    let verifying_start = Instant::now();
    let cycles_start = unsafe { rdtsc() };

    let verifier = Verifier::new(h_list);
    let verifying_result = verifier.check_proof(&zkp_output.shuffle_proof, &commit_list, commit_prime_list);
    assert!(verifying_result);

    let verifying_cycles = unsafe { rdtsc() } - cycles_start;
    let verifying_time = verifying_start.elapsed();
    println!("Verifying time: {:?}", verifying_time);
    println!("Verifying cycles: {:?}", verifying_cycles);

    // Verify commitment openings
    let commits_start = Instant::now();
    let cycles_start = unsafe { rdtsc() };

    let pedersen = Pedersen::new(&h);
    let commits_result = pedersen.verify_list(&zkp_output.m_list, &zkp_output.r_list, commit_prime_list);
    assert!(commits_result);
    
    let commit_cycles = unsafe { rdtsc() } - cycles_start;
    let commits_time = commits_start.elapsed();
    println!("Commits verifying time: {:?}", commits_time);
    println!("Commits verifying cycles: {:?}", commit_cycles);

    write_json_to_file(&rdv_prime, "./outputs/rdv_prime.json").unwrap();
    write_json_to_file(&rdcv, "./outputs/rdcv.json").unwrap();
    write_json_to_file(&rdcv_prime, "./outputs/rdcv_prime.json").unwrap();
    write_json_to_file(&zkp_output, "./outputs/zkp_output.json").unwrap();

    write_json_to_file(&e2easy.sign(&rdv_prime), "./outputs/rdv_prime.sig").unwrap();
    write_json_to_file(&e2easy.sign(&rdcv), "./outputs/rdcv.sig").unwrap();
    write_json_to_file(&e2easy.sign(&rdcv_prime), "./outputs/rdcv_prime.sig").unwrap();
    write_json_to_file(&e2easy.sign(&zkp_output), "./outputs/zkp_output.sig").unwrap();
}
