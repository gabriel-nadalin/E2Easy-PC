use std::fmt;

use crate::groups::Group;

pub mod e2easy;
pub mod types;
pub mod keys;
pub mod utils;
pub mod groups;
pub mod shuffler;
pub mod verifier;
pub mod io_helpers;

pub const N: usize = 10;

#[derive(Clone, PartialEq)]
pub struct Ciphertext<G: Group>(pub G::Element, pub G::Element);

impl<G: Group> fmt::Debug for Ciphertext<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?},{:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Proof<G: Group>((G::Element, G::Element, G::Element, (G::Element, G::Element), [G::Element; N]), (G::Scalar, G::Scalar, G::Scalar, G::Scalar, [G::Scalar; N], [G::Scalar; N]), [G::Element; N], [G::Element; N]);

impl<G: Group> fmt::Debug for Proof<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("({:?},{:?},{:?},{:?})", self.0, self.1, self.2, self.3).replace(" ", ""))
    }
}