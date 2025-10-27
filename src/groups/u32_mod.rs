use std::fmt;
use std::sync::Arc;

use rand::random_range;

use crate::groups::{Element, Group, Scalar};

#[derive(Clone, PartialEq)]
pub struct U32ModScalar {
    pub value: u32,
    pub group: Arc<U32ModGroup>,
}

#[derive(Clone, PartialEq)]
pub struct U32ModElement {
    pub value: u32,
    pub group: Arc<U32ModGroup>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct U32ModGroup {
    pub p: u32,
    pub q: u32,
    pub g: u32,
}

impl Scalar<U32ModGroup> for U32ModScalar {

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

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
    
    fn group(&self) -> U32ModGroup {
        (*self.group).clone()
    }
}

impl Element<U32ModGroup> for U32ModElement {

    // aqui, atua como operador entre dois elementos de um grupo multiplicativo (Z_p*), por isso multiplicacao e nao adicao
    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.group.p, other.group.p, "Elements must be from the same group");
        U32ModElement {
            value: modmul(self.value, other.value, self.group.p),
            group: self.group.clone(),
        }
    }

    // pelo mesmo motivo da funcao `add()`, utiliza exp ao inves de mul
    fn mul_scalar(&self, scalar: &<U32ModGroup as Group>::Scalar) -> Self {
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

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
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
            value: 1,
            group: Arc::new(self.clone()),
        }
    }

    fn zero(&self) -> Self::Scalar {
        U32ModScalar {
            value: 0,
            group: Arc::new(self.clone()),
        }
    }

    fn one(&self) -> Self::Scalar {
        U32ModScalar {
            value: 1,
            group: Arc::new(self.clone()),
        }
    }

    fn random_element(&self) -> Self::Element {
        U32ModElement {
            value: random_range(2..self.p),
            group: Arc::new(self.clone()),
        }
    }

    fn random_scalar(&self) -> Self::Scalar {
        U32ModScalar {
            value: random_range(2..self.q),
            group: Arc::new(self.clone()),
        }
    }
    
    fn mul_generator(&self, scalar: &Self::Scalar) -> Self::Element {
        U32ModElement {
            value: modexp(self.g, scalar.value, self.p),
            group: Arc::new(self.clone()),
        }
    }

    fn element_from_bytes(&self, bytes: &[u8]) -> Self::Element {
        let mut arr = [0; 4];
        for (i, val) in bytes.into_iter().take(4).enumerate() {
            arr[i] = *val;
        }
        U32ModElement {
            value: u32::from_be_bytes(arr) % self.p,
            group: Arc::new(self.clone()),
        }
    }

    fn scalar_from_bytes(&self, bytes: &[u8]) -> Self::Scalar {
        let mut arr = [0; 4];
        for (i, val) in bytes.into_iter().take(4).enumerate() {
            arr[i] = *val;
        }
        U32ModScalar {
            value: u32::from_be_bytes(arr) % self.q,
            group: Arc::new(self.clone()),
        }
    }
}

impl fmt::Debug for U32ModElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.value.to_be_bytes()))
    }
}

impl fmt::Debug for U32ModScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.value.to_be_bytes()))
    }
}

impl U32ModGroup {
    pub fn new(p: u32, q: u32, g: u32) -> Arc<Self> {
        Arc::new(U32ModGroup { p, q, g })
    }

    pub fn get_group_params() -> (u32, u32, u32) {
        let (p, q) = safe_prime(2_u32.pow(31)).unwrap();
        let mut g = random_range(2..p-1);
        g = modexp(g, 2, p);
        (p, q, g)
    }
}

// mod arithmetic utils

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f32).sqrt() as u32 + 1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// retorna um primo seguro e a ordem do grupo (p = 2*q + 1)
pub fn safe_prime(size: u32) -> Option<(u32, u32)> {
    loop {
        let q = random_range(0..size);
        let p = 2 * q + 1;

        if is_prime(p) && is_prime(q) {
            return Some((p, q))
        }
    }
}

pub fn modmul(a: u32, b: u32, modulo: u32) -> u32 {
    ((a as u64 * b as u64) % modulo as u64) as u32
}

pub fn modinv(a: u32, modulo: u32) -> Option<u32> {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = modulo as i64;
    let mut new_r = a as i64;

    while new_r != 0 {
        let q = r / new_r;
        (t, new_t) = (new_t, t - q * new_t);
        (r, new_r) = (new_r, r - q * new_r);
    }

    if r > 1 {
        return None
    }
    if t < 0 {
        t = t + modulo as i64;
    }
    return Some(t as u32)
}

pub fn modexp(base: u32, mut exp: u32, modulo: u32) -> u32 {
    if modulo == 1 {
        return 0
    }
    let mut b = base as u64;
    let m = modulo as u64;
    let mut result = 1;

    b %= m;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * b % m;
        }
        b = b * b % m;
        exp /= 2;
    }
    result as u32
}
