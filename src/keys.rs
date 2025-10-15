use std::sync::Arc;

use rand08::rngs::OsRng;
use ed25519_dalek::{Signature, SignatureError, Signer, SigningKey, Verifier, VerifyingKey};
use crate::{groups::{Element, Group}, Ciphertext};

pub struct SignatureKeys {
    pub sk: SigningKey,
    pub vk: VerifyingKey,
}

pub struct EncryptionKeys<G: Group> {
    pub sk: SecretKey<G>,
    pub pk: PublicKey<G>,
}

pub struct SecretKey<G: Group> {
    pub scalar: G::Scalar,
}

#[derive(Debug, Clone, Copy)]
pub struct PublicKey<G: Group> {
    pub element: G::Element,
}

// pub struct SigningKey<G: Group> {
//     pub scalar: G::Scalar,
// }

// pub struct VerificationKey<G: Group> {
//     pub element: G::Element,
// }

impl<G: Group> EncryptionKeys<G> {
    pub fn encrypt(&self, m: &G::Element, r: &G::Scalar) -> Ciphertext<G> {
        self.pk.encrypt(m, r)
    }

    pub fn encrypt_bytes(&self, bytes: Vec<u8>) -> Vec<u8> {
        todo!()
    }

    pub fn decrypt(&self, c: &Ciphertext<G>) -> G::Element {
        self.sk.decrypt(c)
    }
}

impl SignatureKeys {
    pub fn sign(&mut self, m: Vec<u8>) -> Signature {
        self.sk.sign(&m)
    }

    pub fn verify(&self, m: Vec<u8>, signature: &Signature) -> Result<(), SignatureError> {
        self.vk.verify(&m, signature)
    }
}

impl<G: Group> SecretKey<G> {
    pub fn new(group: Arc<G>) -> Self {
        Self {
            scalar: group.random_scalar()
        }
    }

    pub fn decrypt(&self, c: &Ciphertext<G>) -> G::Element {
        let Ciphertext(c1, c2) = c;
        c1.add(&c2.mul_scalar(&self.scalar).inv())
    }

    pub fn public_key(&self, group: Arc<G>) -> PublicKey<G> {
        PublicKey {
            element: group.mul_generator(&self.scalar)
        }
    }
}

impl<G: Group> PublicKey<G> {
    pub fn encrypt(&self, m: &G::Element, r: &G::Scalar) -> Ciphertext<G> {
        let c1 = m.add(&self.element.mul_scalar(&r));
        let c2 = self.element.group().mul_generator(&r);
        Ciphertext(c1, c2)
    }
}

// impl<G: Group> SigningKey<G> {
//     pub fn sign(&self, m: &G::Element) -> Vec<u8> {
//         todo!()
//     }
// }

// impl<G: Group> VerificationKey<G> {
//     pub fn verify(&self, m: &str) -> bool {
//         todo!()
//     }
// }

pub fn keygen<G: Group>(group: Arc<G>) -> (EncryptionKeys<G>, SignatureKeys) {
    let sk = SecretKey::new(group.clone());
    let pk = sk.public_key(group);

    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    
    let enc_keys = EncryptionKeys {sk, pk};
    let ver_keys = SignatureKeys {sk: signing_key, vk: verifying_key};

    (enc_keys, ver_keys)
}