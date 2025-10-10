pub mod utils;
pub mod traits;
pub mod groups;
pub mod el_gamal;
pub mod shuffler;
pub mod verifier;

pub const N: usize = 500;

type Ciphertext = (u32, u32);
type Proof = ((u32, u32, u32, (u32, u32), [u32; N]), (u32, u32, u32, u32, [u32; N], [u32; N]), [u32; N], [u32; N]);