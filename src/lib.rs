use crypto_bigint::{Uint, NonZero, modular::{MontyForm, MontyParams}};

pub mod utils;
pub mod traits;
// pub mod groups;
pub mod el_gamal;
pub mod shuffler;
pub mod verifier;

pub const N: usize = 10;

// NIST standard
pub const GROUP_SIZE: usize = 3072; // Length in bits
// Ignored becaus there is no difference in execution time
// pub const KEY_SIZE: usize = 256; // Length in bits
                                 //
// pub type Exponent = Uint<{KEY_SIZE/64}>; // Convert bits to LIMBS
pub type Number = Uint<{GROUP_SIZE/64}>; // Convert bits to LIMBS
pub type NumberNZ = NonZero<Number>; // Convert bits to LIMBS
pub type ModNumber = MontyForm<{GROUP_SIZE/64}>;
pub type ModNumberParams = MontyParams<{GROUP_SIZE/64}>;
pub type Ciphertext = (ModNumber, ModNumber);
type Proof = ((ModNumber, ModNumber, ModNumber, (ModNumber, ModNumber), [ModNumber; N]), (Number, Number, Number, Number, [Number; N], [Number; N]), [ModNumber; N], [ModNumber; N]);
