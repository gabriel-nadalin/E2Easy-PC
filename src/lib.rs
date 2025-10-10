use crate::groups::Group;

pub mod e2easy;
pub mod types;
pub mod keys;
pub mod utils;
pub mod groups;
pub mod el_gamal;
pub mod shuffler;
pub mod verifier;

pub const N: usize = 10;

type Ciphertext<G: Group> = (G::Element, G::Element);
type Proof<G: Group> = ((G::Element, G::Element, G::Element, (G::Element, G::Element), [G::Element; N]), (G::Scalar, G::Scalar, G::Scalar, G::Scalar, [G::Scalar; N], [G::Scalar; N]), [G::Element; N], [G::Element; N]);