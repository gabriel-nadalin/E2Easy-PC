use mixnet_rust::{groups::{bigint_mod::BigIntModGroup, traits::Group}, io_helpers::request_user_input, keys::PublicKey, types::Vote, utils::derive_nonces, ModNumber, Number};
use sha2::{Digest, Sha256};

fn main() {
    let (p, q, _g) = mixnet_rust::utils::get_group_params();
    let temp_g = Number::from_be_hex("8F02386269E565E528A755A4E470A90F645886422F784F4CE7B807394B39022234992AD21C1F5B08A67E01011C051BEBB933AC78D76372FBE516138D300FD893BBF512B90ACB9D71E3FF8BD4E51068AF1EAE69E192A68099615D5A17EC384694D8775C836DAB037AEB1028E992F6488AE254CB8D07103E7934B309B027C263D1A5060B923E67FF1EEC95A349E6BE36745F2CF1B26B4E74AD8F7ECE14EFD9C354628A6D63B51626DBB5D75BE7A6E50CFAF3C901FEB761A0ADBEE5B2812BEF748C5A89E1264D62AC7EFD31E3DC50F2D8F84F5CCA9F90EDC4318A0CC0C1A12B4C32E79B96DB22570989E3E8D281E7F43B6F6837C680B18E2B9332D0ADE38138E8E57B6799FBF80F5C3999DEA2C24CAFF10505F32EFBEDFE31BF990D20683AD5610AD9763889EBD178F49009289E535A64CB8488DCC740BB929E39CCC8D2AF18728E5F65577C7059C923957757978D1ECC5047825DAA84E111D4E60CF68B00D8E0B04EA4AF37B22C452B6CBF2743265BBFA8784C5F5A5440C5D58431072CA2E15CB6").to_nz().unwrap();
    let g = ModNumber::new(&temp_g, p).square(); 
    
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