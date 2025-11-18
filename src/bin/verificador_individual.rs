use mixnet_rust::{io_helpers::request_user_input, pedersen::Pedersen, types::{StoredElement, TrackingCode, Vote}, utils::derive_nonces};
use sha2::{Digest, Sha256};
use p256::elliptic_curve::group::prime::PrimeCurveAffine;

fn main() {
    
    println!("Verificando um voto individual");
    // let h = request_user_input("Insira h: ");
    let tc: String = request_user_input("Insira o tracking code: ");
    let voto1 = request_user_input("Insira o voto para presidente: ");
    let voto2 = request_user_input("Insira o voto para governador: ");
    let previous_hash = request_user_input("Insira o hash anterior: ");
    let nonce = request_user_input("Insira o nonce: ");
    let timestamp = request_user_input("Insira o carimbo de tempo: ");
    println!("Verificando o voto...");
    
    let h = serde_json::from_str::<StoredElement>("\"0335EB803C924AA6E9A434BCFF41A00FA8C7877A368C76E48D275E8BD5C217516C\"").unwrap().to_curve();
    let pedersen = Pedersen::new(&h);
    let votes = vec![Vote::new(0, voto1.parse::<u8>().unwrap()), Vote::new(1, voto2.parse::<u8>().unwrap())];
    let previous_hash = TrackingCode(hex::decode(previous_hash).unwrap());
    let seed = hex::decode(nonce).unwrap();
    let nonces = derive_nonces(&seed, votes.len());

    let mut to_hash = (previous_hash, timestamp, Vec::new());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let encoded_vote = vote.to_scalar();
        let committed_vote = pedersen.commit(&encoded_vote, &nonce);

        to_hash.2.push(committed_vote.to_affine());
    }

    assert_eq!(tc, hex::encode(Sha256::digest(serde_json::to_string(&to_hash).unwrap().as_bytes())), "Resultado: Erro! O voto NÃO foi gerado corretamente.");
    println!("Resultado: Sucesso! O voto foi gerado corretamente.");
}