use mixnet_rust::{e2easy::E2Easy, groups::{u32_mod::U32ModGroup, traits::{Element, Group, Scalar}}, types::{TrackingCode, Vote, Ciphertext}, utils::{derive_nonces}};
use sha2::{Digest, Sha256};

fn main() {
    let (p, q, g) = U32ModGroup::get_group_params();
    let group = U32ModGroup::new(p, q, g);

    let mut e2easy = E2Easy::new(group.clone());




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
    println!("{:#?}\n\n", e2easy.vote_table);





    let votes = vec![
        Vote { contest: 0, choice: 2},
        Vote { contest: 1, choice: 2},
        Vote { contest: 2, choice: 2},
    ];

    let (tc, ts) = e2easy.vote(votes);
    println!("tracking code: {:?}", tc);

    let chal = e2easy.challenge();
    let (last_tc, votes, nonce_seed) = chal.clone();
    let nonces = derive_nonces(&*group, &nonce_seed.to_bytes(), votes.len());

    let mut to_hash = last_tc.0.clone();
    to_hash.extend_from_slice(ts.as_bytes());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let vote_element = e2easy.group.element_from_bytes(&vote.to_bytes());

        let enc_vote_bytes = e2easy.enc_keys.encrypt(&vote_element, &nonce).to_bytes();
        
        to_hash.extend_from_slice(&enc_vote_bytes);
    }
    
    assert_eq!(tc, TrackingCode(Sha256::digest(to_hash).to_vec()));

    println!("vote challenged!");
    println!("{:#?} {:#?}\n\n", chal, e2easy.vote_table);





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
    println!("{:#?}\n\n", e2easy.vote_table);





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
    println!("{:#?}\n\n", e2easy.vote_table);





    let votes = vec![
        Vote { contest: 0, choice: 5},
        Vote { contest: 1, choice: 5},
        Vote { contest: 2, choice: 5},
    ];

    let (tc, ts) = e2easy.vote(votes);
    println!("tracking code: {:?}", tc);

    let chal = e2easy.challenge();
    let (last_tc, votes, nonce_seed) = chal.clone();
    let nonces = derive_nonces(&*group, &nonce_seed.to_bytes(), votes.len());

    let mut to_hash = last_tc.0.clone();
    to_hash.extend_from_slice(ts.as_bytes());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let vote_element = e2easy.group.element_from_bytes(&vote.to_bytes());
        
        let enc_vote_bytes = e2easy.enc_keys.encrypt(&vote_element, &nonce).to_bytes();
        
        to_hash.extend_from_slice(&enc_vote_bytes);
    }
    
    assert_eq!(tc, TrackingCode(Sha256::digest(to_hash).to_vec()));

    println!("vote challenged!");
    println!("{:#?} {:#?}\n\n", chal, e2easy.vote_table);

    
    e2easy.tally();
}