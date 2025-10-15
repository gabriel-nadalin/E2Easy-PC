use mixnet_rust::{el_gamal::ElGamal, shuffler::Shuffler, utils::*, verifier::Verifier, N};
use rand::random_range;

fn main() {
    let (p, q) = safe_prime(2_u32.pow(31)).unwrap();
    let mut g = random_range(2..p-1);
    g = modexp(g, 2, p);

    let mut el_gamal = ElGamal::new(p, q, g);
    el_gamal.keygen();

    let mut h_list: [u32; N] = [0; N];
    for i in 0..N {
        let mut h = random_range(2..p-1);
        h = modexp(h, 2, p);
        h_list[i] = h;
    }

    let shuffler = Shuffler::new(p, q, g, h_list, el_gamal.pk());

    let plaintext_list: [u32; N] = core::array::from_fn(|i| modexp(i as u32 + 1, 2, p));
    let ciphertext_list_1: [(u32, u32); N] = core::array::from_fn(|i| el_gamal.encrypt(plaintext_list[i]));
    
    println!("plaintext: {:?}", plaintext_list);
    println!("ciphertext: {:?}", ciphertext_list_1);

    let (ciphertext_list_2, random_list, psi) = shuffler.gen_shuffle(ciphertext_list_1);
    let proof = shuffler.gen_proof(
        ciphertext_list_1,
        ciphertext_list_2,
        random_list,
        psi
    );
    println!("shuffled: {:?}", ciphertext_list_2);
    println!("proof: {:?}", proof);

    let verifier = Verifier::new(p, q, g, h_list);
    let result = verifier.check_proof(proof, ciphertext_list_1, ciphertext_list_2, el_gamal.pk());
    println!("result: {result}");

    let decrypted_list: [u32; N] = core::array::from_fn(|i| el_gamal.decrypt(ciphertext_list_2[i]));
    println!("shuffled & decrypted: {:?}", decrypted_list);
}
