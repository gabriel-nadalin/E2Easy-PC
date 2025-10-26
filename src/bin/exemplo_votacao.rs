use std::io::{self, Write};
use mixnet_rust::io_helpers::request_user_input;

use mixnet_rust::{e2easy::E2Easy, groups::{u32_mod::U32ModGroup, Element}, types::Vote};

const CHALLENGE: &str = "2";
const CAST: &str = "1";
fn main () {
    println!("\n\n--------- Iniciando urna eletronica... ------------");

    // let (p, q) = safe_prime(2_u32.pow(31)).unwrap();
    // let mut g = random_range(2..p-1);
    // g = modexp(g, 2, p);

    let (p, q, g) = (3864258863, 1932129431, 3051949095);

    let group = U32ModGroup::new(p, q, g);

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

