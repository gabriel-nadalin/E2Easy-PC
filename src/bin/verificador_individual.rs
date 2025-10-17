use mixnet_rust::{groups::{u32_mod::U32ModGroup, Element, Group}, io_helpers::request_user_input, keys::PublicKey, types::Vote, Ciphertext};
use sha2::{Digest, Sha256};

fn main() {
    let (p, q, g) = (3864258863, 1932129431, 3051949095);

    let group = U32ModGroup::new(p, q, g);


    println!("Verificando um voto individual");
    let pk = request_user_input("Insira a chave pública: ");
    let tc = request_user_input("Insira o tracking code: ");
    let voto1 = request_user_input("Insira o voto para presidente: ");
    let voto2 = request_user_input("Insira o voto para governador: ");
    let previous_hash = request_user_input("Insira o hash anterior: ");
    let nonce1 = request_user_input("Insira o nonce 1: ");
    let nonce2 = request_user_input("Insira o nonce 2: ");
    let timestamp = request_user_input("Insira o carimbo de tempo: ");
    println!("Verificando o voto...");

    let pk: PublicKey<U32ModGroup> = PublicKey{element: group.deserialize_to_element(hex::decode(pk).unwrap())};
    let votes = vec![Vote::new(0, voto1.parse::<u8>().unwrap()), Vote::new(1, voto2.parse::<u8>().unwrap())];
    let previous_hash = hex::decode(previous_hash).unwrap();
    let nonces = vec![group.deserialize_to_scalar(hex::decode(nonce1).unwrap()), group.deserialize_to_scalar(hex::decode(nonce2).unwrap())];

    let mut to_hash = previous_hash;
    to_hash.extend_from_slice(timestamp.as_bytes());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let encoded = group.deserialize_to_element(vote.to_bytes());
        let Ciphertext(c1, c2) = pk.encrypt(&encoded, &nonce);

        let enc_vote = [c1.serialize(), c2.serialize()].concat();
        
        to_hash.extend_from_slice(&enc_vote);
    }

    assert_eq!(tc, hex::encode(Sha256::digest(to_hash)), "Resultado: Erro! O voto NÃO foi gerado corretamente.");
    println!("Resultado: Sucesso! O voto foi gerado corretamente.");
}