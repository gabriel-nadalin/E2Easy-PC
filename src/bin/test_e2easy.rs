use mixnet_rust::{e2easy::E2Easy, io_helpers::write_json_to_file, pedersen::Pedersen, types::{TrackingCode, Vote}, utils::{derive_nonces, hash, random_element}};
use sha2::{Digest, Sha256};

fn main() {
    // let (p, q, g) = U32ModGroup::get_group_params();
    // let group = U32ModGroup::new(p, q, g);
    // let pedersen = Pedersen::new(h)

    let h = random_element();
    let mut e2easy = E2Easy::new(&h);
    let pedersen = Pedersen::new(&h);


    let votes = vec![
        Vote { contest: 0, choice: 1},
        Vote { contest: 1, choice: 1},
        Vote { contest: 2, choice: 1},
    ];

    let tc = e2easy.vote(votes);
    println!("tracking code: {:?}", tc);
    
    let sig = e2easy.cast();
    println!("signature: {:?}", sig);

    println!("vote cast!");
    // println!("{:#?}\n\n", e2easy.vote_table);





    let votes = vec![
        Vote { contest: 0, choice: 2},
        Vote { contest: 1, choice: 2},
        Vote { contest: 2, choice: 2},
    ];

    let (tc, ts) = e2easy.vote(votes.clone());
    println!("tracking code: {:?}", tc);

    let chal = e2easy.challenge();
    let (last_tc, _commits, nonce_seed) = chal.clone();
    let nonces = derive_nonces(&nonce_seed.to_bytes(), votes.len());
    
    let mut to_hash = (last_tc.clone(), ts.clone(), Vec::new());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let encoded_vote = vote.to_scalar();
        let committed_vote = pedersen.commit(&encoded_vote, &nonce);

        to_hash.2.push(committed_vote.to_affine());
    }
    
    assert_eq!(tc, TrackingCode(Sha256::digest(serde_json::to_string(&to_hash).unwrap().as_bytes()).to_vec()));

    println!("vote challenged!");
    // println!("{:#?} {:#?}\n\n", chal, e2easy.vote_table);





    let votes = vec![
        Vote { contest: 0, choice: 3},
        Vote { contest: 1, choice: 3},
        Vote { contest: 2, choice: 3},
    ];

    let tc = e2easy.vote(votes);
    println!("tracking code: {:?}", tc);
    
    let sig = e2easy.cast();
    println!("signature: {:?}", sig);

    println!("vote cast!");
    // println!("{:#?}\n\n", e2easy.vote_table);





    let votes = vec![
        Vote { contest: 0, choice: 4},
        Vote { contest: 1, choice: 4},
        Vote { contest: 2, choice: 4},
    ];

    let tc = e2easy.vote(votes);
    println!("tracking code: {:?}", tc);
    
    let sig = e2easy.cast();
    println!("signature: {:?}", sig);

    println!("vote cast!");
    // println!("{:#?}\n\n", e2easy.vote_table);





    let votes = vec![
        Vote { contest: 0, choice: 5},
        Vote { contest: 1, choice: 5},
        Vote { contest: 2, choice: 5},
    ];

    let (tc, ts) = e2easy.vote(votes.clone());
    println!("tracking code: {:?}", tc);

    let chal = e2easy.challenge();
    let (last_tc, _commits, nonce_seed) = chal.clone();
    let nonces = derive_nonces(&nonce_seed.to_bytes(), votes.len());
    
    let mut to_hash = (last_tc.clone(), ts.clone(), Vec::new());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let encoded_vote = vote.to_scalar();
        let committed_vote = pedersen.commit(&encoded_vote, &nonce);

        to_hash.2.push(committed_vote.to_affine());
    }
    
    assert_eq!(tc, TrackingCode(hash(to_hash)));

    println!("vote challenged!");
    // println!("{:#?} {:#?}\n\n", chal, e2easy.vote_table);

    
    let (rdv, rdcv, rdcv_prime, zkp_output) = e2easy.tally();

    write_json_to_file(&rdv, "./outputs/rdv.json").unwrap();
    write_json_to_file(&rdcv, "./outputs/rdcv.json").unwrap();
    write_json_to_file(&rdcv_prime, "./outputs/rdcv_prime.json").unwrap();
    write_json_to_file(&zkp_output, "./outputs/zkp_output.json").unwrap();
}