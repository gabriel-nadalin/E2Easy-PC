use std::fmt;
use std::sync::Arc;

use crypto_bigint::{RandomMod, rand_core::OsRng};

use crate::groups::{Element, Group, Scalar};
use crate::{Number, NumberNZ, ModNumber, ModNumberParams, SIZE};


#[derive(Clone, PartialEq)]
pub struct U32ModScalar {
    pub value: Number,
    pub group: Arc<U32ModGroup>,
}

#[derive(Clone, PartialEq)]
pub struct U32ModElement {
    pub value: ModNumber,
    pub group: Arc<U32ModGroup>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct U32ModGroup {
    pub p: ModNumberParams,
    pub q: NumberNZ,
    pub g: ModNumber,
}

impl Scalar<U32ModGroup> for U32ModScalar {

    fn add(&self, other: &Self) -> Self {
        U32ModScalar {
            value: self.value.add_mod(&other.value, &self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn sub(&self, other: &Self) -> Self {
        U32ModScalar {
            value: self.value.sub_mod(&other.value, &self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn mul(&self, other: &Self) -> Self {
        U32ModScalar {
            value: self.value.mul_mod(&other.value, &self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn neg(&self) -> Self {
        U32ModScalar {
            value: self.value.neg_mod(&self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn inv(&self) -> Self {
        let inv = self.value.inv_mod(&self.group.p.modulus().as_nz_ref())
            .expect("No modular inverse exists for this value");
        U32ModScalar {
            value: inv,
            group: self.group.clone(),
        }
    }
}

impl Element<U32ModGroup> for U32ModElement {

    // Aqui, atua como operador entre dois elementos de um grupo multiplicativo (Z_p*), por isso multiplicacao e nao adicao
    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.group.p, other.group.p, "Elements must be from the same group");
        U32ModElement {
            value: self.value.mul(&other.value),
            group: self.group.clone(),
        }
    }

    // Pelo mesmo motivo da funcao `add()`, utiliza exp ao inves de mul
    fn mul_scalar(&self, scalar: &<U32ModGroup as Group>::Scalar) -> Self {
        U32ModElement {
            value: self.value.pow(&scalar.value),
            group: self.group.clone(),
        }
    }

    fn inv(&self) -> Self {
        // There is no .expect for ConstCtOption<MontyForm<4>>...
        let inv = self.value.inv().unwrap();
            // .expect("No modular inverse exists for this value");
        U32ModElement {
            value: inv,
            group: self.group.clone(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        self.value.retrieve().to_be_bytes().to_vec()
    }
    
    fn group(&self) -> U32ModGroup {
        (*self.group).clone()
    }
}

impl Group for U32ModGroup {
    type Element = U32ModElement;
    type Scalar = U32ModScalar;

    fn identity(&self) -> Self::Element {
        U32ModElement {
            value: ModNumber::one(self.p),
            group: Arc::new(self.clone()),
        }
    }

    fn zero(&self) -> Self::Scalar {
        U32ModScalar {
            value: Number::ZERO,
            group: Arc::new(self.clone()),
        }
    }

    fn one(&self) -> Self::Scalar {
        U32ModScalar {
            value: Number::ONE,
            group: Arc::new(self.clone()),
        }
    }

    fn random_element(&self) -> Self::Element {
        // Asserts you don't get the value 1
        let mut rand: Number;
        loop {
            rand = Number::random_mod(&mut OsRng, &self.p.modulus().as_nz_ref());
            if rand > Number::ONE {
                break;
            }
        }
        let element_rand = ModNumber::new(&rand, self.p).square(); 
        U32ModElement {
            value: element_rand,
            group: Arc::new(self.clone()),
        }
    }

    fn random_scalar(&self) -> Self::Scalar {
        // Asserts you don't get the value 1
        let mut rand: Number;
        loop {
            rand = Number::random_mod(&mut OsRng, &self.q);
            if rand > Number::ONE {
                break;
            }
        }
        U32ModScalar {
            value: rand,
            group: Arc::new(self.clone()),
        }
    }
    
    fn mul_generator(&self, scalar: &Self::Scalar) -> Self::Element {
        U32ModElement {
            value: self.g.pow(&scalar.value),
            group: Arc::new(self.clone()),
        }
    }

    fn deserialize_to_element(&self, bytes: Vec<u8>) -> Self::Element {
        let mut arr = [0; SIZE/8];
        for (i, val) in bytes.into_iter().take(SIZE/8).rev().enumerate() {
            arr[SIZE/8-1 - i] = val;
        }
        U32ModElement {
            value: ModNumber::new(&Number::from_be_slice(&arr), self.p),
            group: Arc::new(self.clone()),
        }
    }

    fn deserialize_to_scalar(&self, bytes: Vec<u8>) -> Self::Scalar {
        let mut arr = [0; SIZE/8];
        for (i, val) in bytes.into_iter().take(SIZE/8).rev().enumerate() {
            arr[SIZE/8-1 - i] = val;
        }
        U32ModScalar {
            value: Number::from_be_slice(&arr).add_mod(&Number::ZERO, &self.q),
            group: Arc::new(self.clone()),
        }
    }
}

impl fmt::Debug for U32ModElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.value.retrieve().to_be_bytes()))
    }
}

impl fmt::Debug for U32ModScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.value.to_be_bytes()))
    }
}

impl U32ModGroup {
    pub fn new(p: ModNumberParams, q: NumberNZ, g: ModNumber) -> Arc<Self> {
        Arc::new(U32ModGroup { p, q, g })
    }
}
