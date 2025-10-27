use mixnet_rust::{groups::{bigint_mod::BigIntModGroup, traits::{Element, Group}}, io_helpers::request_user_input, keys::PublicKey, types::Vote, utils::derive_nonces, types::Ciphertext};
use sha2::{Digest, Sha256};

fn main() {
    let (p, q, g) = mixnet_rust::utils::get_group_params();

    let group = BigIntModGroup::new(p, q, g);


    println!("Verificando um voto individual");
    let pk = request_user_input("Insira a chave pública: ");
    let tc = request_user_input("Insira o tracking code: ");
    let voto1 = request_user_input("Insira o voto para presidente: ");
    let voto2 = request_user_input("Insira o voto para governador: ");
    let previous_hash = request_user_input("Insira o hash anterior: ");
    let nonce = request_user_input("Insira o nonce: ");
    let timestamp = request_user_input("Insira o carimbo de tempo: ");
    println!("Verificando o voto...");

    let pk: PublicKey<BigIntModGroup> = PublicKey{element: group.element_from_bytes(&hex::decode(pk).unwrap())};
    let votes = vec![Vote::new(0, voto1.parse::<u8>().unwrap()), Vote::new(1, voto2.parse::<u8>().unwrap())];
    let previous_hash = hex::decode(previous_hash).unwrap();
    let seed = hex::decode(nonce).unwrap();
    let nonces = derive_nonces(&*group, &seed, votes.len());

    let mut to_hash = previous_hash;
    to_hash.extend_from_slice(timestamp.as_bytes());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let vote_element = group.element_from_bytes(&vote.to_bytes());
        
        let enc_vote_bytes = pk.encrypt(&vote_element, &nonce).to_bytes();
        
        to_hash.extend_from_slice(&enc_vote_bytes);
    }

    assert_eq!(tc, hex::encode(Sha256::digest(to_hash)), "Resultado: Erro! O voto NÃO foi gerado corretamente.");
    println!("Resultado: Sucesso! O voto foi gerado corretamente.");
}