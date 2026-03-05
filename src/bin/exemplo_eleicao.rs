use mixnet_rust::{io_helpers::write_json_to_file, types::config::*, utils::random_element};


const N: u32 = 5000;
const CONTESTS: u32 = 6;
fn main() {
    println!("Criando uma nova eleição com {} cargos e {} eleitores", CONTESTS, N);

    let config = ElectionConfig {
        crypto: CryptoParams {
            h: random_element(),
            h_list: (0..N).into_iter().map(|_| random_element()).collect::<Vec<_>>(),
        },
        contests: (0..CONTESTS)
            .into_iter()
            .map(|i| ContestInfo {
                contest_id: i,
                name: format!("contest_{i}"),
                num_choices: 5,
            }).collect(),
    };
    
    write_json_to_file(&config, "./config/election_config.json").unwrap();
    println!("Arquivos criados em /config/");
}