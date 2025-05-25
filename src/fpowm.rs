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

//! Module to wrap the functions related to the fixed-sized modulo exponential in gmpmee
//!
//! The structure [FPowmTable] permits to initialized the precomputation and to perform the exponentiate
//! ```
//! use rug::Integer;
//! use rug_gmpmee::fpowm::FPowmTable;
//! let p = Integer::from(13);
//! let b = Integer::from(7);
//! let e = Integer::from(4);
//! let tab = FPowmTable::init_precomp(&b, &p, 16, 16).unwrap();
//! let res = tab.fpowm(&e);
//! assert_eq!(res, b.pow_mod(&e, &p).unwrap());
//! ```
//!
//! It is possible to used a cache table, as static variable. The cache must be initiliazed once and
//! cannot be changed anymore
//! ```
//! use rug::Integer;
//! use rug_gmpmee::fpowm::{cache_init_precomp, cache_fpown, cache_base_modulus};
//! let p = Integer::from(13);
//! let b = Integer::from(7);
//! let e = Integer::from(4);
//! assert!(cache_base_modulus().is_none());
//! let res_init = cache_init_precomp(&b, &p, 16, 1024);
//! assert!(res_init.is_ok());
//! assert!(res_init.unwrap());
//! assert_eq!(cache_base_modulus().unwrap(), (&b, &p));
//! assert_eq!(cache_fpown(&e).unwrap(),b.pow_mod(&e, &p).unwrap());
//! ```

use crate::GmpMEEError;
use gmpmee_sys::{
    gmpmee_fpowm, gmpmee_fpowm_clear, gmpmee_fpowm_init, gmpmee_fpowm_init_precomp,
    gmpmee_fpowm_precomp, gmpmee_fpowm_tab, gmpmee_spowm_tab,
};
use rug::Integer;
use std::sync::OnceLock;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum FPownError {
    #[error("{variable} cannot be casted to i64 (in {method}): {source}")]
    ExponentCast {
        method: &'static str,
        variable: &'static str,
        source: std::num::TryFromIntError,
    },
}

/// Structure containing the structure of the table to precompute of fixed-sized modulo exponential
///
/// The structure implementes `Sync` and `Send` for the caching function
pub struct FPowmTable {
    inner: gmpmee_fpowm_tab,
}

unsafe fn get_empty_gmpmee_fpowm_tab() -> gmpmee_fpowm_tab {
    gmpmee_fpowm_tab {
        spowm_table: gmpmee_spowm_tab {
            len: 0,
            block_width: 0,
            tabs_len: 0,
            tabs: std::ptr::null_mut(),
            modulus: Integer::new().into_raw(),
        },
        stretch: 0,
    }
}

impl FPowmTable {
    /// Wrap `gmpmee_init``
    pub fn init(
        modulus: &Integer,
        block_width: usize,
        exponent_bitlen: usize,
    ) -> Result<Self, GmpMEEError> {
        let block_width_i64: i64 =
            block_width
                .try_into()
                .map_err(|e| FPownError::ExponentCast {
                    method: "FPowmTable::init",
                    variable: "block_width",
                    source: e,
                })?;
        let exponent_bitlen_i64: i64 =
            exponent_bitlen
                .try_into()
                .map_err(|e| FPownError::ExponentCast {
                    method: "FPowmTable::init",
                    variable: "exponent_bitlen",
                    source: e,
                })?;
        unsafe {
            let mut tab = get_empty_gmpmee_fpowm_tab();
            let t_ptr = &mut tab;
            gmpmee_fpowm_init(
                t_ptr,
                modulus.as_raw(),
                block_width_i64,
                exponent_bitlen_i64,
            );
            Ok(Self { inner: *t_ptr })
        }
    }

    /// Wrap `gmpmee_init_precomp``
    pub fn init_precomp(
        base: &Integer,
        modulus: &Integer,
        block_width: usize,
        exponent_bitlen: usize,
    ) -> Result<Self, GmpMEEError> {
        let block_width_i64: i64 =
            block_width
                .try_into()
                .map_err(|e| FPownError::ExponentCast {
                    method: "FPowmTable::init_precomp",
                    variable: "block_width",
                    source: e,
                })?;
        let exponent_bitlen_i64: i64 =
            exponent_bitlen
                .try_into()
                .map_err(|e| FPownError::ExponentCast {
                    method: "FPowmTable::init_precomp",
                    variable: "exponent_bitlen",
                    source: e,
                })?;
        unsafe {
            let mut tab = get_empty_gmpmee_fpowm_tab();
            let t_ptr = &mut tab;
            gmpmee_fpowm_init_precomp(
                t_ptr,
                base.as_raw(),
                modulus.as_raw(),
                block_width_i64,
                exponent_bitlen_i64,
            );
            Ok(Self { inner: *t_ptr })
        }
    }

    /// Wrap `gmpmee_precomp``
    pub fn precomp(&mut self, base: &Integer) {
        unsafe { gmpmee_fpowm_precomp(&mut self.inner, base.as_raw()) }
    }

    /// Wrap `gmpmee_fpowm``
    pub fn fpowm(&self, exponent: &Integer) -> Integer {
        let mut res = Integer::new();
        unsafe {
            let z_ptr = res.as_raw_mut();
            gmpmee_fpowm(z_ptr, &self.inner, exponent.as_raw());
        }
        res
    }
}

impl Drop for FPowmTable {
    fn drop(&mut self) {
        unsafe { gmpmee_fpowm_clear(&mut self.inner) }
    }
}

static CACHE_FPOWM_TABLE: OnceLock<FPownMTableStatic> = OnceLock::new();

unsafe impl Sync for FPowmTable {}
unsafe impl Send for FPowmTable {}

struct FPownMTableStatic {
    pub table: FPowmTable,
    modulus: Integer,
    base: Integer,
}

fn is_cache_initialized() -> bool {
    CACHE_FPOWM_TABLE.get().is_some()
}

/// Initialize the cache with the given parameters.
///
/// The cache cannot be changed anymore
pub fn cache_init_precomp(
    base: &Integer,
    modulus: &Integer,
    block_width: usize,
    exponent_bitlen: usize,
) -> Result<bool, GmpMEEError> {
    if !is_cache_initialized() {
        let _ = CACHE_FPOWM_TABLE.set(FPownMTableStatic {
            table: FPowmTable::init_precomp(base, modulus, block_width, exponent_bitlen)?,
            modulus: modulus.clone(),
            base: base.clone(),
        });
        return Ok(true);
    }
    Ok(false)
}

/// Calculate `gmpmee_fpowm` using the cache
///
/// If the cache is not initialized, then return `None`
pub fn cache_fpown(exponent: &Integer) -> Option<Integer> {
    if !is_cache_initialized() {
        return None;
    }
    Some(CACHE_FPOWM_TABLE.get().unwrap().table.fpowm(exponent))
}

/// Return the base and the modulus as tuple used for the initialization of the cache
///
/// If the cache is not initialized, then return `None`
pub fn cache_base_modulus() -> Option<(&'static Integer, &'static Integer)> {
    CACHE_FPOWM_TABLE
        .get()
        .map(|cache| (&cache.base, &cache.modulus))
}

#[cfg(test)]
mod test {
    use super::*;
    use rayon::iter::IntoParallelRefIterator;
    use rayon::prelude::*;
    use rug::rand::RandState;
    use std::time::SystemTime;

    #[test]
    fn test_init() {
        let res = FPowmTable::init(&Integer::from(11), 16, 16);
        assert!(res.is_ok());
    }

    #[test]
    fn test_init_precomp() {
        let res = FPowmTable::init_precomp(&Integer::from(8), &Integer::from(11), 16, 16);
        assert!(res.is_ok());
    }

    #[test]
    fn test_precomp() {
        let mut res = FPowmTable::init(&Integer::from(11), 16, 16).unwrap();
        res.precomp(&Integer::from(8));
    }

    #[test]
    fn test_fpown() {
        let p = Integer::from(13);
        let b = Integer::from(7);
        let e = Integer::from(4);
        let tab = FPowmTable::init_precomp(&b, &p, 16, 16).unwrap();
        let res = tab.fpowm(&e);
        assert_eq!(res, b.pow_mod(&e, &p).unwrap())
    }

    #[test]
    fn test_fpown_big() {
        let p =  Integer::from(Integer::parse_radix(
            "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867", 16
        ).unwrap());
        let mut rand = RandState::new();
        let b = Integer::from(Integer::random_bits(2048, &mut rand));
        let e = Integer::from(Integer::random_bits(1024, &mut rand));
        let tab = FPowmTable::init_precomp(&b, &p, 16, 16).unwrap();
        let res = tab.fpowm(&e);
        assert_eq!(res, b.pow_mod(&e, &p).unwrap())
    }

    #[test]
    fn test_performance() {
        let p =  Integer::from(Integer::parse_radix(
            "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867", 16
        ).unwrap());
        let mut rand = RandState::new();
        let b = Integer::from(Integer::random_bits(2048, &mut rand));
        let b2 = b.clone();
        let e = Integer::from(Integer::random_bits(1024, &mut rand));
        let begin_rug = SystemTime::now();
        let res_rug = b2.pow_mod(&e, &p).unwrap();
        let duration_rug = begin_rug.elapsed().unwrap();
        //let begin_fpowm_with_precomp = SystemTime::now();
        let tab = FPowmTable::init_precomp(&b, &p, 16, 1024).unwrap();
        let begin_fpowm = SystemTime::now();
        let res_fpowm = tab.fpowm(&e);
        let duration_fpowm = begin_fpowm.elapsed().unwrap();
        //let duration_fpowm_with_precomp = begin_fpowm_with_precomp.elapsed().unwrap();
        assert_eq!(res_fpowm, res_rug);
        assert!(
            duration_rug > duration_fpowm,
            "The duration of fpown (={} ms) is bigger than duration with rug (={} ms)",
            duration_fpowm.as_millis(),
            duration_rug.as_millis()
        );
        /*println!("Duration rug: {} micro s", duration_rug.as_micros());
        println!("Duration fpowm: {} micro s", duration_fpowm.as_micros());
        println!(
            "Duration fpowm with init: {} micro s",
            duration_fpowm_with_precomp.as_micros()
        );*/
    }

    #[test]
    fn test_cache() {
        let p =  Integer::from(Integer::parse_radix(
            "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867", 16
        ).unwrap());
        let mut rand = RandState::new();
        let base = Integer::from(Integer::random_bits(2048, &mut rand));
        assert!(cache_base_modulus().is_none());
        let res_init = cache_init_precomp(&base, &p, 16, 1024);
        assert!(res_init.is_ok());
        assert!(res_init.unwrap());
        assert_eq!(cache_base_modulus().unwrap(), (&base, &p));
        let nb_exps = 100;
        let mut exponents = vec![];
        (0..nb_exps)
            .for_each(|_| exponents.push(Integer::from(Integer::random_bits(1024, &mut rand))));
        let begin_rug = SystemTime::now();
        let res_rug = exponents
            .par_iter()
            .map(|e| Integer::from(base.pow_mod_ref(e, &p).unwrap()))
            .collect::<Vec<_>>();
        let duration_rug = begin_rug.elapsed().unwrap();
        let begin_fpowm = SystemTime::now();
        let res_fpowm = exponents
            .par_iter()
            .map(|e| cache_fpown(e).unwrap())
            .collect::<Vec<_>>();
        let duration_fpowm = begin_fpowm.elapsed().unwrap();
        assert_eq!(res_fpowm.len(), res_rug.len());
        for res in res_fpowm.iter() {
            assert!(res_rug.contains(res));
        }
        assert!(
            duration_rug > duration_fpowm,
            "The duration of fpown (={} ms) is bigger than duration with rug (={} ms)",
            duration_fpowm.as_millis(),
            duration_rug.as_millis()
        );
        //println!("Duration rug: {} micro s", duration_rug.as_micros());
        //println!("Duration fpowm: {} micro s", duration_fpowm.as_micros());
    }
}
