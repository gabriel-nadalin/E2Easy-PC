use std::fmt;

use crypto_bigint::{Uint, NonZero, modular::{MontyForm, MontyParams}};

use crate::groups::traits::Group;

pub mod e2easy;
pub mod types;
pub mod keys;
pub mod utils;
pub mod groups;
pub mod shuffler;
pub mod verifier;
pub mod io_helpers;

pub const N: usize = 3000;
pub const SIZE: usize = 3072;
pub type Number = Uint<{SIZE/64}>;
pub type NumberNZ = NonZero<Number>;
pub type ModNumber = MontyForm<{SIZE/64}>;
pub type ModNumberParams = MontyParams<{SIZE/64}>;

#[derive(Clone, PartialEq)]
pub struct Ciphertext<G: Group>(pub G::Element, pub G::Element);

impl<G: Group> fmt::Debug for Ciphertext<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?},{:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct ShuffleProof<G: Group>((G::Element, G::Element, G::Element, (G::Element, G::Element), Vec<G::Element>), (G::Scalar, G::Scalar, G::Scalar, G::Scalar, Vec<G::Scalar>, Vec<G::Scalar>), Vec<G::Element>, Vec<G::Element>);

impl<G: Group> fmt::Debug for ShuffleProof<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("({:?},{:?},{:?},{:?})", self.0, self.1, self.2, self.3).replace(" ", ""))
    }
}
