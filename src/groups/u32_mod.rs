use std::sync::Arc;

use bincode::{Encode, Decode};
use rand::random_range;

use crate::groups::{Element, Group, Scalar};
use crate::utils::{modmul, modexp, modinv};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct U32ModScalar {
    pub value: u32,
    pub group: Arc<U32ModGroup>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct U32ModElement {
    pub value: u32,
    pub group: Arc<U32ModGroup>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct U32ModGroup {
    pub p: u32,
    pub q: u32,
    pub g: u32,
}

impl Scalar<Arc<U32ModGroup>> for U32ModScalar {

    fn add(&self, other: &Self) -> Self {
        U32ModScalar {
            value: (self.value + other.value) % self.group.q,
            group: self.group.clone(),
        }
    }
    
    fn sub(&self, other: &Self) -> Self {
        U32ModScalar {
            value: (self.value - other.value + self.group.q) % self.group.q,
            group: self.group.clone(),
        }
    }
    
    fn mul(&self, other: &Self) -> Self {
        U32ModScalar {
            value: modmul(self.value, other.value, self.group.q),
            group: self.group.clone(),
        }
    }
    
    fn neg(&self) -> Self {
        U32ModScalar {
            value: (self.group.q - self.value) % self.group.q,
            group: self.group.clone(),
        }
    }
    
    fn inv(&self) -> Self {
        let inv = modinv(self.value, self.group.p)
            .expect("No modular inverse exists for this value");
        U32ModScalar {
            value: inv,
            group: self.group.clone(),
        }
    }
}

impl Element<Arc<U32ModGroup>> for U32ModElement {

    // aqui, atua como operador entre dois elementos de um grupo multiplicativo (Z_p*), por isso multiplicacao e nao adicao
    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.group.p, other.group.p, "Elements must be from the same group");
        U32ModElement {
            value: modmul(self.value, other.value, self.group.p),
            group: self.group.clone(),
        }
    }

    // pelo mesmo motivo da funcao `add()`, utiliza exp ao inves de mul
    fn mul_scalar(&self, scalar: &<Arc<U32ModGroup> as Group>::Scalar) -> Self {
        U32ModElement {
            value: modexp(self.value, scalar.value, self.group.p),
            group: self.group.clone(),
        }
    }

    fn inv(&self) -> Self {
        let inv = modinv(self.value, self.group.p)
            .expect("No modular inverse exists for this value");
        U32ModElement {
            value: inv,
            group: self.group.clone(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
    
    fn group(&self) -> Arc<U32ModGroup> {
        self.group.clone()
    }
}

impl Group for Arc<U32ModGroup> {
    type Element = U32ModElement;
    type Scalar = U32ModScalar;

    fn identity(&self) -> Self::Element {
        U32ModElement {
            value: 1,
            group: self.clone(),
        }
    }

    fn zero(&self) -> Self::Scalar {
        U32ModScalar {
            value: 0,
            group: self.clone(),
        }
    }

    fn one(&self) -> Self::Scalar {
        U32ModScalar {
            value: 1,
            group: self.clone(),
        }
    }

    fn random_element(&self) -> Self::Element {
        U32ModElement {
            value: random_range(0..self.p),
            group: self.clone(),
        }
    }

    fn random_scalar(&self) -> Self::Scalar {
        U32ModScalar {
            value: random_range(0..self.q),
            group: self.clone(),
        }
    }
    
    fn mul_generator(&self, scalar: &Self::Scalar) -> Self::Element {
        U32ModElement {
            value: modexp(self.g, scalar.value, self.p),
            group: self.clone(),
        }
    }

    fn deserialize_to_element(&self, bytes: Vec<u8>) -> Self::Element {
        let mut arr = [0; 4];
        for (i, val) in bytes.into_iter().take(4).enumerate() {
            arr[i] = val;
        }
        U32ModElement {
            value: u32::from_be_bytes(arr),
            group: self.clone(),
        }
    }
}


impl U32ModGroup {
    pub fn new(p: u32, q: u32, g: u32) -> Arc<Self> {
        Arc::new(U32ModGroup { p, q, g })
    }

    pub fn element_from_u32(self: &Arc<Self>, e: u32) -> U32ModElement {
        U32ModElement {
            value: e,
            group: self.clone()
        }
    }

    pub fn scalar_from_u32(self: &Arc<Self>, s: u32) -> U32ModScalar {
        U32ModScalar {
            value: s,
            group: self.clone()
        }
    }
}