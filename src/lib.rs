use p256::{AffinePoint, ProjectivePoint};

pub mod e2easy;
pub mod types;
pub mod utils;
pub mod pedersen;
pub mod shuffler;
pub mod verifier;
pub mod io_helpers;

pub const G: ProjectivePoint = ProjectivePoint::GENERATOR;
pub const SIZE: usize = 256;
pub type Element = AffinePoint;
pub type Scalar = p256::Scalar;
