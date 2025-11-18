use p256::elliptic_curve::group::prime::PrimeCurveAffine;
use mixnet_rust::{e2easy::E2Easy, io_helpers::{request_user_input, write_json_to_file}, types::{StoredElement, Vote}};

const CHALLENGE: &str = "2";
// const CAST: &str = "1";
fn main () {
    println!("\n\n--------- Iniciando urna eletronica... ------------");

    let h = serde_json::from_str::<StoredElement>("\"0335EB803C924AA6E9A434BCFF41A00FA8C7877A368C76E48D275E8BD5C217516C\"").unwrap().to_curve();
    let mut e2easy = E2Easy::new(&h);

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

        let votes = vec![Vote{ contest: 0, choice: vote1 }, Vote{ contest: 1, choice: vote2 }];

        let (tracking_code, timestamp) = e2easy.vote(votes);
        println!("\nObrigado. Aqui está o seu código de rastreio: {}", hex::encode(tracking_code.0));

        let challenge_or_cast = request_user_input("Deseja (1) lançar o voto ou (2) desafiar a urna? ");
        if challenge_or_cast == CHALLENGE {
            let (previous_hash, _votes, nonce) = e2easy.challenge();

            println!("Aqui estão os dados para o desafio:");
            println!("    hash anterior: {}", hex::encode(previous_hash.0));
            println!("    nonce: {:?}", hex::encode(nonce.to_bytes()));
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

    let (rdv, rdcv, rdcv_prime, zkp_output) = e2easy.tally();

    write_json_to_file(&rdv, "./outputs/rdv.json").unwrap();
    write_json_to_file(&rdcv, "./outputs/rdcv.json").unwrap();
    write_json_to_file(&rdcv_prime, "./outputs/rdcv_prime.json").unwrap();
    write_json_to_file(&zkp_output, "./outputs/zkp_output.json").unwrap();
    
    println!("Arquivos criados em /outputs/");
    println!("--------- Urna eletrônica encerrada ------------\n\n");
}

