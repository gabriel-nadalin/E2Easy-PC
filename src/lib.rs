use p256::ProjectivePoint;

// pub mod e2easy;
pub mod types;
// pub mod keys;
pub mod utils;
// pub mod groups;
pub mod pedersen;
pub mod shuffler;
pub mod verifier;
pub mod io_helpers;

pub const N: usize = 3000;
pub const G: ProjectivePoint = ProjectivePoint::GENERATOR;
pub const SIZE: usize = 256;
pub type Element = ProjectivePoint;
pub type Scalar = p256::Scalar;
