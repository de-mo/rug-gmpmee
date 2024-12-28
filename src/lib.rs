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

pub mod fpowm;
pub mod miller_rabin;
pub mod spown;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GmpMEEError {
    #[error("Error in parameters of spowm: {0}")]
    /// Error in the parameters of spowm
    SPowmParameters(String),
    #[error("Error in parameters of fpowm: {0}")]
    /// Error in the parameters of fpwon and related functions
    FPowmParameters(String),
    #[error("Error with cache fpowm: {0}")]
    /// Error in the cache function
    FPowmCache(String),
}
