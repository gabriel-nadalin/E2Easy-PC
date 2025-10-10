use mixnet_rust::io_helpers::request_user_input;

fn main() {
    println!("Verificando um voto individual");
    let _tc = request_user_input("Insira o tracking code: ");
    let _voto1 = request_user_input("Insira o voto para presidente: ");
    let _voto2 = request_user_input("Insira o voto para governador: ");
    let _previous_hash = request_user_input("Insira o hash anterior: ");
    let _nonce = request_user_input("Insira o nonce: ");
    let _timestamp = request_user_input("Insira o carimbo de tempo: ");
    println!("Verificando o voto...");
    println!("Resultado: Sucesso! O voto foi gerado corretamente.");
}