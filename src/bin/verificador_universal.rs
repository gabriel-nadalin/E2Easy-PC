use mixnet_rust::Element;
use mixnet_rust::io_helpers::read_json;
use mixnet_rust::pedersen::Pedersen;
use mixnet_rust::types::{InfoContest, RDCV, RDCVPrime, RDVPrime, TrackingCode, Vote, ZKPOutput};
use mixnet_rust::utils::hash;
use p256::ecdsa::Signature;
use p256::ecdsa::signature::Verifier;

fn main() {
    println!("Verificando as eleições em /outputs");

    let info_contest: InfoContest = read_json("./outputs/info_contest.json").unwrap();

    let rdv_prime: RDVPrime = read_json("./outputs/rdv_prime.json").unwrap();
    let rdcv: RDCV = read_json("./outputs/rdcv.json").unwrap();
    let rdcv_prime: RDCVPrime = read_json("./outputs/rdcv_prime.json").unwrap();
    let zkp_output: ZKPOutput = read_json("./outputs/zkp_output.json").unwrap();

    let rdv_prime_sig: Signature = read_json("./outputs/rdv_prime.sig").unwrap();
    let rdcv_sig: Signature = read_json("./outputs/rdcv.sig").unwrap();
    let rdcv_prime_sig: Signature = read_json("./outputs/rdcv_prime.sig").unwrap();
    let zkp_output_sig: Signature = read_json("./outputs/zkp_output.sig").unwrap();

    let tail = rdcv.tail();
    let commit_list = rdcv.votes();
    let head = rdcv.head().clone().unwrap();

    let commit_prime_list = rdcv_prime.entries();

    let pi = zkp_output.shuffle_proof;

    let h = info_contest.crypto.h;
    let h_list: Vec<Element> = info_contest.crypto.h_list.iter().take(rdcv_prime.entries().len()).cloned().collect();

    println!("Verificando assinaturas");

    let vk = zkp_output.verifying_key;
    vk.verify(&std::fs::read("./outputs/rdv_prime.json").unwrap(), &rdv_prime_sig).unwrap();
    vk.verify(&std::fs::read("./outputs/rdcv.json").unwrap(), &rdcv_sig).unwrap();
    vk.verify(&std::fs::read("./outputs/rdcv_prime.json").unwrap(), &rdcv_prime_sig).unwrap();
    vk.verify(&std::fs::read("./outputs/zkp_output.json").unwrap(), &zkp_output_sig).unwrap();

    println!("Verificando hashchain");

    let mut prev_hash = tail.clone();

    for entry in rdcv.entries() {
        let to_hash = (prev_hash, &entry.timestamp, &entry.committed_votes);
        let tc = TrackingCode(hash(&to_hash));
        assert_eq!(tc, entry.tracking_code);
        prev_hash = tc;
    }
    let to_hash = (prev_hash, b"CLOSE");
    assert_eq!(TrackingCode(hash(&to_hash)), head);

    println!("Verificando prova de embaralhamento");

    let verifier = mixnet_rust::verifier::Verifier::new(h_list);
    assert!(verifier.check_proof(&pi, &commit_list, commit_prime_list));

    println!("Verificando abertura dos compromissos");
    let pedersen = Pedersen::new(&h);
    let m_list = zkp_output.m_list;
    let r_list = zkp_output.r_list;

    assert!(pedersen.verify_list(&m_list, &r_list, commit_prime_list));

    let votes = m_list.iter().map(|m| Vote::from_scalar(*m)).collect();
    let rdv_prime_m = RDVPrime::new(votes);

    assert_eq!(rdv_prime, rdv_prime_m);

    println!("Eleição verificada com sucesso!");
}