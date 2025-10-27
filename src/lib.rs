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

pub const N: usize = 10;
pub const SIZE: usize = 3072;
pub type Number = Uint<{SIZE/64}>;
pub type NumberNZ = NonZero<Number>;
pub type ModNumber = MontyForm<{SIZE/64}>;
pub type ModNumberParams = MontyParams<{SIZE/64}>;