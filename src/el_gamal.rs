use crate::{utils::*, Ciphertext, Number, NumberNZ, ModNumber, ModNumberParams};

pub struct ElGamal {
    p: ModNumberParams,
    q: NumberNZ,
    g: ModNumber,
    sk: Number,
    pk: ModNumber
}

impl ElGamal {
    pub fn new(group: ModNumberParams, order: NumberNZ, generator: ModNumber) -> Self {
        Self {
            p: group,
            q: order,
            g: generator,
            sk: Number::ZERO,
            pk: ModNumber::zero(group)
        }
    }

    pub fn pk(&self) -> ModNumber {
        self.pk
    }

    pub fn keygen(&mut self) {
        self.sk = get_exponent(&self.q).unwrap();
        self.pk = self.g.pow(&self.sk);
    }

    pub fn encrypt(&self, m: Number) -> Ciphertext {
        let r = get_exponent(&self.q).unwrap();
        let c1 = ModNumber::new(&m, self.p).mul(&self.pk.pow(&r));
        let c2 = self.g.pow(&r);
        return (c1, c2)
    }

    pub fn re_encrypt(&self, e: Ciphertext) -> Ciphertext {
        self.multiply_ciphertexts(e, self.encrypt(Number::ONE))
    }

    pub fn decrypt(&self, ciphertext: Ciphertext) -> Number {
        let (c1, c2) = ciphertext;
        let msg = c1.mul(&c2.pow(&self.sk).inv().unwrap());
        return msg.retrieve()
    }

    pub fn multiply_ciphertexts(&self, c: Ciphertext, d: Ciphertext) -> Ciphertext {
        let (c1, c2) = c;
        let (d1, d2) = d;
        let r1 = c1.mul(&d1);
        let r2 = c2.mul(&d2);
        return (r1, r2)
    }
}
