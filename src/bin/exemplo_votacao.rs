use std::io::{self, Write};
use mixnet_rust::io_helpers::request_user_input;

use mixnet_rust::{ModNumber, Number};
use mixnet_rust::{e2easy::E2Easy, groups::{bigint_mod::BigIntModGroup, traits::Element}, types::Vote};

const CHALLENGE: &str = "2";
const CAST: &str = "1";
fn main () {
    println!("\n\n--------- Iniciando urna eletronica... ------------");

    // let (p, q) = safe_prime(2_u32.pow(31)).unwrap();
    // let mut g = random_range(2..p-1);
    // g = modexp(g, 2, p);

    let (p, q, _g) = mixnet_rust::utils::get_group_params();
    let temp_g = Number::from_be_hex("8F02386269E565E528A755A4E470A90F645886422F784F4CE7B807394B39022234992AD21C1F5B08A67E01011C051BEBB933AC78D76372FBE516138D300FD893BBF512B90ACB9D71E3FF8BD4E51068AF1EAE69E192A68099615D5A17EC384694D8775C836DAB037AEB1028E992F6488AE254CB8D07103E7934B309B027C263D1A5060B923E67FF1EEC95A349E6BE36745F2CF1B26B4E74AD8F7ECE14EFD9C354628A6D63B51626DBB5D75BE7A6E50CFAF3C901FEB761A0ADBEE5B2812BEF748C5A89E1264D62AC7EFD31E3DC50F2D8F84F5CCA9F90EDC4318A0CC0C1A12B4C32E79B96DB22570989E3E8D281E7F43B6F6837C680B18E2B9332D0ADE38138E8E57B6799FBF80F5C3999DEA2C24CAFF10505F32EFBEDFE31BF990D20683AD5610AD9763889EBD178F49009289E535A64CB8488DCC740BB929E39CCC8D2AF18728E5F65577C7059C923957757978D1ECC5047825DAA84E111D4E60CF68B00D8E0B04EA4AF37B22C452B6CBF2743265BBFA8784C5F5A5440C5D58431072CA2E15CB6").to_nz().unwrap();
    let g = ModNumber::new(&temp_g, p).square(); 
    
    let group = BigIntModGroup::new(p, q, g);

    let mut e2easy = E2Easy::new(group);

    println!("Chave pública de cifração: {}", hex::encode(e2easy.enc_keys.pk.element.to_bytes()));

    loop {
        let is_new_voter = request_user_input("\nReceber novo voto? (s/n): ");
        if is_new_voter == "n" {
            break;
        }

        let vote1 = request_user_input("Por favor, digite seu voto para presidente: ")
            .parse::<u8>().unwrap();
        println!("Voto confirmado: {}", vote1);

        let vote2 = request_user_input("Por favor, digite seu voto para governador: ")
            .parse::<u8>().unwrap();
        println!("Voto confirmado: {}", vote2);

        let votes = vec![Vote{ contest: 0, choice: vote1}, Vote{ contest: 1, choice: vote2}];

        let (tracking_code, timestamp) = e2easy.vote(votes);
        println!("\nObrigado. Aqui está o seu código de rastreio: {}", hex::encode(tracking_code.0));

        let challenge_or_cast = request_user_input("Deseja (1) lançar o voto ou (2) desafiar a urna? ");
        if challenge_or_cast == CHALLENGE {
            let (previous_hash, _votes, nonce) = e2easy.challenge();

            println!("Aqui estão os dados para o desafio:");
            println!("    hash anterior: {}", hex::encode(previous_hash.0));
            println!("    nonce: {:?}", nonce);
            println!("    carimbo de tempo: {}", timestamp);
            println!("Voto descartado. Vote novamente.")
        } else {
            let tc_signature = e2easy.cast();
            println!("Aqui está a assinatura do seu código de rastreio:\n{}", hex::encode(tc_signature.to_bytes()));
            println!("Voto lançado! Obrigado por votar.");
        }
    }

    println!("\nEncerrando urna eletrônica...");
    println!("Misturando os votos e gerando as provas...");
    println!("Arquivos criados em /outputs/");
    println!("--------- Urna eletrônica encerrada ------------\n\n");
}

