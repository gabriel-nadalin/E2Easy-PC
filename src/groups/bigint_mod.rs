use std::fmt;
use std::sync::Arc;

use crypto_bigint::{RandomMod, rand_core::OsRng};

use crate::groups::traits::{Element, Group, Scalar};
use crate::{Number, NumberNZ, ModNumber, ModNumberParams, SIZE};


#[derive(Clone, PartialEq)]
pub struct BigIntModScalar {
    pub value: Number,
    pub group: Arc<BigIntModGroup>,
}

#[derive(Clone, PartialEq)]
pub struct BigIntModElement {
    pub value: ModNumber,
    pub group: Arc<BigIntModGroup>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BigIntModGroup {
    pub p: ModNumberParams,
    pub q: NumberNZ,
    pub g: ModNumber,
}

impl Scalar<BigIntModGroup> for BigIntModScalar {

    fn add(&self, other: &Self) -> Self {
        BigIntModScalar {
            value: self.value.add_mod(&other.value, &self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn sub(&self, other: &Self) -> Self {
        BigIntModScalar {
            value: self.value.sub_mod(&other.value, &self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn mul(&self, other: &Self) -> Self {
        BigIntModScalar {
            value: self.value.mul_mod(&other.value, &self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn neg(&self) -> Self {
        BigIntModScalar {
            value: self.value.neg_mod(&self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn inv(&self) -> Self {
        let inv = self.value.inv_mod(&self.group.p.modulus().as_nz_ref())
            .expect("No modular inverse exists for this value");
        BigIntModScalar {
            value: inv,
            group: self.group.clone(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
    
    fn group(&self) -> BigIntModGroup {
        (*self.group).clone()
    }
}

impl Element<BigIntModGroup> for BigIntModElement {

    // Aqui, atua como operador entre dois elementos de um grupo multiplicativo (Z_p*), por isso multiplicacao e nao adicao
    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.group.p, other.group.p, "Elements must be from the same group");
        BigIntModElement {
            value: self.value.mul(&other.value),
            group: self.group.clone(),
        }
    }

    // Pelo mesmo motivo da funcao `add()`, utiliza exp ao inves de mul
    fn mul_scalar(&self, scalar: &<BigIntModGroup as Group>::Scalar) -> Self {
        BigIntModElement {
            value: self.value.pow(&scalar.value),
            group: self.group.clone(),
        }
    }

    fn inv(&self) -> Self {
        // There is no .expect for ConstCtOption<MontyForm<4>>...
        let inv = self.value.inv().unwrap();
            // .expect("No modular inverse exists for this value");
        BigIntModElement {
            value: inv,
            group: self.group.clone(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.retrieve().to_be_bytes().to_vec()
    }
    
    fn group(&self) -> BigIntModGroup {
        (*self.group).clone()
    }

    fn to_scalar(&self) -> BigIntModScalar {
        BigIntModScalar {
            value: self.value.retrieve(),
            group: self.group.clone(),
        }
    }
}

impl Group for BigIntModGroup {
    type Element = BigIntModElement;
    type Scalar = BigIntModScalar;

    fn identity(&self) -> Self::Element {
        BigIntModElement {
            value: ModNumber::one(self.p),
            group: Arc::new(self.clone()),
        }
    }

    fn zero(&self) -> Self::Scalar {
        BigIntModScalar {
            value: Number::ZERO,
            group: Arc::new(self.clone()),
        }
    }

    fn one(&self) -> Self::Scalar {
        BigIntModScalar {
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
        BigIntModElement {
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
        BigIntModScalar {
            value: rand,
            group: Arc::new(self.clone()),
        }
    }
    
    fn mul_generator(&self, scalar: &Self::Scalar) -> Self::Element {
        BigIntModElement {
            value: self.g.pow(&scalar.value),
            group: Arc::new(self.clone()),
        }
    }

    fn element_from_bytes(&self, bytes: &[u8]) -> Self::Element {
        let mut arr = [0; SIZE/8];
        for (i, val) in bytes.into_iter().take(SIZE/8).rev().enumerate() {
            arr[SIZE/8-1 - i] = *val;
        }
        BigIntModElement {
            value: ModNumber::new(&Number::from_be_slice(&arr), self.p),
            group: Arc::new(self.clone()),
        }
    }

    fn scalar_from_bytes(&self, bytes: &[u8]) -> Self::Scalar {
        let mut arr = [0; SIZE/8];
        for (i, val) in bytes.into_iter().take(SIZE/8).rev().enumerate() {
            arr[SIZE/8-1 - i] = *val;
        }
        BigIntModScalar {
            value: Number::from_be_slice(&arr).add_mod(&Number::ZERO, &self.q),
            group: Arc::new(self.clone()),
        }
    }
}

impl fmt::Debug for BigIntModElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.value.retrieve().to_be_bytes()))
    }
}

impl fmt::Debug for BigIntModScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.value.to_be_bytes()))
    }
}

impl BigIntModGroup {
    pub fn new(p: ModNumberParams, q: NumberNZ, g: ModNumber) -> Arc<Self> {
        Arc::new(BigIntModGroup { p, q, g })
    }
}
