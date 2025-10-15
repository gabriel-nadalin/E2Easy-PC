use std::io::{self, Write};
use mixnet_rust::io_helpers::request_user_input;

const CHALLENGE: &str = "2";
const CAST: &str = "1";
fn main () {
    println!("\n\n--------- Iniciando urna eletronica... ------------");

    loop {
        let is_new_voter = request_user_input("\nReceber novo voto? (s/n): ");
        if is_new_voter == "n" {
            break;
        }

        let vote1 = request_user_input("Por favor, digite seu voto para presidente: ");
        println!("Voto confirmado: {}", vote1);

        let vote2 = request_user_input("Por favor, digite seu voto para governador: ");
        println!("Voto confirmado: {}", vote2);

        let tracking_code = "3e9893e18b881cc8ccf9b9d3117965266668e2bae96ff668c6a4ff8ca78078d4";
        println!("\nObrigado. Aqui está o seu código de rastreio: {}", tracking_code);

        let challenge_or_cast = request_user_input("Deseja (1) lançar o voto ou (2) desafiar a urna? ");
        if challenge_or_cast == CHALLENGE {
            let previous_hash = "727676308869a6c86b8f7d3f8c924431d2f86e4723dc68ce36a58be88b1467de";
            let nonce = "727676308869a6c86b8f7d3f8c924431d2f86e4723dc68ce36a58be88b1467de";
            let timestamp = "2025-10-01 12:00:00";
            println!("Aqui estão os dados para o desafio:");
            println!("    hash anterior: {}", previous_hash);
            println!("    nonce: {}", nonce);
            println!("    carimbo de tempo: {}", timestamp);
            println!("Voto descartado. Vote novamente.")
        } else {
            let tc_signature = "3045022100dff1d77f2a671c5f462f7e1c5b3a8c4ab6b8f1b4e9d3e8e6c7f6a5f4d3c2b1a1022039c3b2a1f0e9d8c7b6a5f4e3d2c1b0a9f8e7d6c5b4a3b2c1d0e9f8e7d6c5b4a3";
            println!("Aqui está a assinatura do seu código de rastreio:\n{}", tc_signature);
            println!("Voto lançado! Obrigado por votar.");
        }
    }

    println!("\nEncerrando urna eletrônica...");
    println!("Misturando os votos e gerando as provas...");
    println!("Arquivos criados em /outputs/");
    println!("--------- Urna eletrônica encerrada ------------\n\n");
}

