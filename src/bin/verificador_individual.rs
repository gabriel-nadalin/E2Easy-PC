use mixnet_rust::{io_helpers::{read_json, request_user_input}, pedersen::Pedersen, types::*, utils::{derive_nonces, hash}};

fn main() {
    
    println!("Verificando um voto individual");
    let tc: String = request_user_input("Insira o tracking code: ");
    let voto1 = request_user_input("Insira o voto para presidente: ");
    let voto2 = request_user_input("Insira o voto para governador: ");
    let previous_hash = request_user_input("Insira o hash anterior: ");
    let nonce = request_user_input("Insira o nonce: ");
    let timestamp = request_user_input("Insira o carimbo de tempo: ");
    println!("Verificando o voto...");
    
    let election_config: ElectionConfig = read_json("./config/election_config.json").unwrap();
    let h= election_config.crypto.h;
    let pedersen = Pedersen::new(&h);
    let votes = vec![Vote::new(0, voto1.parse::<u8>().unwrap()), Vote::new(1, voto2.parse::<u8>().unwrap())];
    let seed = hex::decode(nonce).unwrap();
    let nonces = derive_nonces(&seed, votes.len());

    let mut to_hash = (previous_hash, timestamp, Vec::new());

    for (vote, nonce) in votes.iter().zip(nonces) {
        let encoded_vote = vote.to_scalar();
        let committed_vote = pedersen.commit(&encoded_vote, &nonce);

        to_hash.2.push(committed_vote);
    }

    assert_eq!(tc, hex::encode_upper(hash(&to_hash)), "Resultado: Erro! O voto NÃO foi gerado corretamente.");
    println!("Resultado: Sucesso! O voto foi gerado corretamente.");
}