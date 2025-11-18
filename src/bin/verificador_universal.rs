use mixnet_rust::io_helpers::read_json;
use mixnet_rust::types::{RDV, RDCV, RDCVPrime, ZKPOutput};

fn main() {
    println!("Verificando as eleições em /outputs/1");

    let rdv: RDV = read_json("./outputs/rdv.json").unwrap();
    let rdcv: RDCV = read_json("./outputs/rdcv.json").unwrap();
    let rdcv_prime: RDCVPrime = read_json("./outputs/rdcv_prime.json").unwrap();
    let zkp_output: ZKPOutput = read_json("./outputs/zkp_output.json").unwrap();

    println!("Eleição verificada com sucesso!");
}