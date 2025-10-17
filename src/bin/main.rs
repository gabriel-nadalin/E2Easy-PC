use mixnet_rust::{Ciphertext, groups::u32_mod::{U32ModGroup, U32ModElement}, keys, shuffler::Shuffler, utils::*, verifier::Verifier, N};
use mixnet_rust::groups::Group;
use rand::random_range;

fn main() {
    let (p, q) = safe_prime(2_u32.pow(31)).unwrap();
    let mut g = random_range(2..p-1);
    g = modexp(g, 2, p);

    let group = U32ModGroup::new(p, q, g);

    let (enc_keys, _sig_keys) = keys::keygen(group.clone());

    let h_list: [U32ModElement; N] = core::array::from_fn(|_| {
        let mut h = random_range(2..p-1);
        h = modexp(h, 2, p);
        group.deserialize_to_element(h.to_be_bytes().to_vec())
    });

    let shuffler = Shuffler::new(group.clone(), h_list.clone(), enc_keys.pk.clone());

    let plaintext_list: [U32ModElement; N] = core::array::from_fn(|i| {
        let val = modexp(i as u32 + 1, 2, p);
        group.deserialize_to_element(val.to_be_bytes().to_vec())
    });

    let ciphertext_list_1: [Ciphertext<U32ModGroup>; N] = core::array::from_fn(|i| {
        let r = group.random_scalar();
        enc_keys.encrypt(&plaintext_list[i], &r)
    });
    
    println!("plaintext: {:?}", plaintext_list);
    println!("ciphertext: {:?}", ciphertext_list_1);

    let (ciphertext_list_2, random_list, psi) = shuffler.gen_shuffle(ciphertext_list_1.clone());
    let proof = shuffler.gen_proof(
        ciphertext_list_1.clone(),
        ciphertext_list_2.clone(),
        random_list.clone(),
        psi
    );
    println!("shuffled: {:?}", ciphertext_list_2);
    println!("proof: {:?}", proof);

    let verifier = Verifier::new(group.clone(), h_list.clone());
    let result = verifier.check_proof(proof, ciphertext_list_1, ciphertext_list_2.clone(), enc_keys.pk.clone());
    println!("result: {result}");

    let decrypted_list: [u32; N] = core::array::from_fn(|i| enc_keys.sk.decrypt(&ciphertext_list_2[i].clone()).value);
    println!("shuffled & decrypted: {:?}", decrypted_list);
}
