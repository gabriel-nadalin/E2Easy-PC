use mixnet_rust::{io_helpers::write_json_to_file, types::config::*, utils::random_element};


const N: u32 = 5000;
const CONTESTS: u32 = 6;
const OPTIONS: u32 = 4;

fn main() {
    println!("Criando uma nova eleição com {} cargos, {} candidatos por cargo e {} votos", CONTESTS, OPTIONS, N);

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
                options: (0..OPTIONS)
                    .into_iter()
                    .map(|i| OptionInfo {
                        option_id: i,
                        name: format!("choice_{i}")
                    }).collect()
            }).collect(),
    };
    
    write_json_to_file(&config, "./config/election_config.json").unwrap();
    println!("Arquivos criados em /config/");
}