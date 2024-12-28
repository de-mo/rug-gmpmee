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

//! Module to wrap the function `gmpmee_spowm`
use gmpmee_sys::gmpmee_spowm;
use rug::Integer;

use crate::GmpMEEError;

/// Multi exponential module.
///
/// Formula: prod_{i=0}^{n} b_i^{e_i} mod m
///
/// The number of bases and exponents must be the same
pub fn spowm(
    bases: &[Integer],
    exponents: &[Integer],
    modulus: &Integer,
) -> Result<Integer, GmpMEEError> {
    if bases.len() != exponents.len() {
        return Err(GmpMEEError::SPowmParameters(format!(
            "Len of bases {} is not the same than len of exponents {}",
            bases.len(),
            exponents.len()
        )));
    }
    let bases_raw = bases.iter().map(|b| b.as_raw()).collect::<Vec<_>>();
    let exponents_raw = exponents.iter().map(|b| b.as_raw()).collect::<Vec<_>>();
    let mut res = Integer::new();
    let len: i64 = bases.len().try_into().map_err(|e| {
        GmpMEEError::SPowmParameters(format!(
            "exponentlen of bases cannot be casted to i64 (in init): {}",
            e
        ))
    })?;
    let bases_ptr = bases_raw[0];
    let exponents_ptr = exponents_raw[0];
    unsafe {
        gmpmee_spowm(
            res.as_raw_mut(),
            bases_ptr,
            exponents_ptr,
            len,
            modulus.as_raw(),
        );
    };
    Ok(res)
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use rug::rand::RandState;

    use super::*;

    pub fn expected_spown(bases: &[Integer], exponents: &[Integer], modulus: &Integer) -> Integer {
        bases
            .iter()
            .zip(exponents.iter())
            .map(|(b, e)| Integer::from(b.pow_mod_ref(e, modulus).unwrap()))
            .fold(Integer::ONE.clone(), |acc, v| (acc * v) % modulus)
    }

    #[test]
    fn test_1() {
        let bases = [Integer::from(2)];
        let exponents = [Integer::from(4)];
        let modulus = Integer::from(13);
        let res = spowm(&bases, &exponents, &modulus).unwrap();
        assert_eq!(res, Integer::from(3))
    }

    #[test]
    fn test_2() {
        let bases = [Integer::from(5), Integer::from(7)];
        let exponents = [Integer::from(3), Integer::from(9)];
        let modulus = Integer::from(13);
        let res = spowm(&bases, &exponents, &modulus).unwrap();
        assert_eq!(res, Integer::from(12))
    }

    #[test]
    fn test_5() {
        let bases = [
            Integer::from(5),
            Integer::from(7),
            Integer::from(8),
            Integer::from(11),
            Integer::from(12),
        ];
        let exponents = [
            Integer::from(3),
            Integer::from(9),
            Integer::from(4),
            Integer::from(12),
            Integer::from(2),
        ];
        let modulus = Integer::from(13);
        let res = spowm(&bases, &exponents, &modulus).unwrap();
        assert_eq!(res, expected_spown(&bases, &exponents, &modulus))
    }

    #[test]
    fn test_performance() {
        let p =  Integer::from(Integer::parse_radix(
            "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867", 16
        ).unwrap());
        let mut rand = RandState::new();
        let len = 100;
        let mut bases = vec![];
        (0..len).for_each(|_| bases.push(Integer::from(Integer::random_bits(3072, &mut rand))));
        let mut exponents = vec![];
        (0..len).for_each(|_| exponents.push(Integer::from(Integer::random_bits(3072, &mut rand))));
        let begin_rug = SystemTime::now();
        let res_rug = expected_spown(&bases, &exponents, &p);
        let duration_rug = begin_rug.elapsed().unwrap();
        let begin_spowm = SystemTime::now();
        let res_spowm = spowm(&bases, &exponents, &p).unwrap();
        let duration_spowm = begin_spowm.elapsed().unwrap();
        assert_eq!(res_spowm, res_rug);
        assert!(
            duration_rug > duration_spowm,
            "The duration of spown (={} ms) is bigger than duration with rug (={} ms)",
            duration_spowm.as_millis(),
            duration_rug.as_millis()
        );
        //println!("Duration rug: {} ms", duration_rug.as_millis());
        //println!("Duration spowm: {} ms", duration_spowm.as_millis());
    }
}
