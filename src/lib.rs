// Copyright © 2024 Denis Morel

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

//! The rug-gmpmee crate provides an implementation for [rug](https://docs.rs/rug/latest/rug/) of
//! the [GMP Modular Exponentiation Extension (GMPMEE)](https://github.com/verificatum/verificatum-gmpmee),
//! which is a minor extension of [GMP](https://gmplib.org/). It adds simultaneous modular exponentiation
//! and fixed base modular exponentiation functionality to the set of integer functions (the mpz-functions),
//! as well as special purpose primality testing routines.
//!
//! It contains the following implementations:
//! - Multi-exponentation (`spowm`)
//! - Fixed base exponentiation (`fpowm`). It contains a possibility to cache the precomputation table
//! - Miller-Rabin primality test
//!
//! The rub-gmpmee crate is free software: you can redistribute it and/or modify it under the terms of the
//! GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License,
//! or (at your option) any later version. See the full text of the [LICENSE](LICENSE.md) for details.
//!
//! # Using rug-gmpmee
//! See the [gmpmee-sys](https://docs.rs/gmpmee-sys) crate.

pub mod fpowm;
pub mod miller_rabin;
pub mod spown;
use fpowm::FPownError;
use spown::SPownError;
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GmpMEEError {
    #[error("Error in parameters of spowm")]
    SPowmParameters(#[from] SPownError),
    #[error("Error in parameters of fpown: {0}")]
    FPowmParameters(#[from] FPownError),
    #[error("{msg}: {source}")]
    Cast {
        msg: String,
        source: TryFromIntError,
    },
}

#[cfg(target_family = "windows")]
fn usize_to_size_t_type(n: usize) -> Result<i32, TryFromIntError> {
    n.try_into()
}

#[cfg(not(target_family = "windows"))]
fn usize_to_size_t_type(n: usize) -> Result<i64, TryFromIntError> {
    n.try_into()
}
