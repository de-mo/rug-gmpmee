use gmpmee_sys::{gmpmee_millerrabin_rs, gmpmee_millerrabin_safe_rs};
use rug::{rand::RandState, Integer};

pub fn miller_rabin(n: &Integer, reps: i32) -> bool {
    let mut rand = RandState::default();
    !matches!(
        unsafe { gmpmee_millerrabin_rs(rand.as_raw_mut(), n.as_raw(), reps) },
        0
    )
}

pub fn miller_rabin_safe(n: &Integer, reps: i32) -> bool {
    let mut rand = RandState::default();
    !matches!(
        unsafe { gmpmee_millerrabin_safe_rs(rand.as_raw_mut(), n.as_raw(), reps) },
        0
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug_miller_rabin::is_prime;
    use std::time::SystemTime;

    const K: i32 = 16;

    #[test]
    fn test_prime() {
        let prime = Integer::from(0x7fff_ffffu64);
        assert!(miller_rabin(&prime, K));
    }

    #[test]
    fn test_composite() {
        let composite = Integer::from(0xffff_ffff_ffff_ffffu64);
        assert!(!miller_rabin(&composite, K));
    }

    #[test]
    fn test_small_primes() {
        for prime in &[2u8, 3u8, 5u8, 7u8, 11u8, 13u8] {
            assert!(miller_rabin(&Integer::from(*prime), K));
        }
    }

    #[test]
    fn test_big_mersenne_prime() {
        let prime = Integer::from(
            Integer::parse_radix(b"170141183460469231731687303715884105727", 10).unwrap(),
        );

        assert!(miller_rabin(&prime, K));
    }

    #[test]
    fn test_big_wagstaff_prime() {
        let prime = Integer::from(
            Integer::parse_radix(b"56713727820156410577229101238628035243", 10).unwrap(),
        );

        assert!(miller_rabin(&prime, K));
    }

    #[test]
    fn test_big_composite() {
        let prime = Integer::from(
            Integer::parse_radix(b"170141183460469231731687303715884105725", 10).unwrap(),
        );

        assert!(!miller_rabin(&prime, K));
    }

    const BIG_PRIMES: [&str;4] = [
        "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867",
        "5FFB3E665707B0D9C5D3856B9B67D4751425AEB6575F97F697E446856FFCF159105FECE66D2CDE9DEA958966FE67A0D51ECDFC0FCAD3EACA293485FA2FBCC9DF3B055DE51F14B82EA39D3331C6E6B753C331E06DC8F1F0558EFF0D7F928C0EA6961DD02CFC898ECAE9BFA18919F5113B702964B06E58987CEFFEE05F4BBE4CA3F3D702F528B5540D92947F781B12D67E7A4AE1D5AEAF8BB703789C1574B52381908496060E0150CB55A6D1069B02DA73952E7E8B67C9C0E41A89F5E8C5452510DFCADC3276D26010A2C1F4CD18C07BD2B0F8CEA28DE21AA73D1426E3F5862D02EE2C42B636E4679D2BDA16C336C2FA29E8DEC663088BFDB035205785077BB6B01E3D183E05C42A1AAEAC1B3BA635D8911C704C033C15243DDCC44570EDAA6F651FF61BA698664D391698292C2834E9095B17EB3AC38819BE50BA08F417FBF3F3DBAA7A64F9D0E24D50AF0685074D82D17544010B68295BC07340B46519B184E9E0C01513C57E78E07C7D19C0E0A2ED0432449110DCB0766B6A30B2F02BDAAF75",
        "BEDCDE3405B8A18D6C7615FCFF97DB1C29CD2CA69F1BB1432E690E1E947836FC1DE9160D5C2ADEE52ED244F7997ECCE19FF979D00CC3CCE3784DA6C6495D0D87337B24ABB0FD848C79EBBCF298349396FAE4031A3B7EC2BF313CAEF36AB191CAD36D4AEFDFFA87F72DAACB2EA854FFFCCC66E99C2896911EBA93341C006DD3AA4DD06B432B2D3FCD79B5F7C61DED181B734B2DC1C869E498B2647E8C4301DBFD1787F1C7F5E687D118F2A5D410DB73689586377AA9273DEEC051B60DB813DD0C22FAD561BABE3C59CC67EB284387EE6D3F8C38F6A0B34DE82CEF929B853C3B1A52C6CD6B87AA0A882C30F8B716B3687CCB8EB9EC1BF67407C5142315D2BDFFA5D37E0ADB968593BC66A999695DF11B0164B21A62F7A0A7006D49EF8DEB31408E66AD53A4A6BE38F20EF09C84C729A9544EDF854274DC2120CAFA1BC08E20E7C7F1969DCD4C2C08DCB8AB419B6A8B22F1D6F183B1912E54B045C84E95E668D282073EF9216E3106C173FF9A1D29DC445059491209FA9540D06B666611EB5ECE77",
        "5F6E6F1A02DC50C6B63B0AFE7FCBED8E14E696534F8DD8A19734870F4A3C1B7E0EF48B06AE156F729769227BCCBF6670CFFCBCE80661E671BC26D36324AE86C399BD9255D87EC2463CF5DE794C1A49CB7D72018D1DBF615F989E5779B558C8E569B6A577EFFD43FB96D56597542A7FFE663374CE144B488F5D499A0E0036E9D526E835A195969FE6BCDAFBE30EF68C0DB9A596E0E434F24C59323F462180EDFE8BC3F8E3FAF343E88C7952EA086DB9B44AC31BBD54939EF76028DB06DC09EE86117D6AB0DD5F1E2CE633F59421C3F7369FC61C7B5059A6F41677C94DC29E1D8D296366B5C3D5054416187C5B8B59B43E65C75CF60DFB3A03E28A118AE95EFFD2E9BF056DCB42C9DE3354CCB4AEF88D80B2590D317BD0538036A4F7C6F598A0473356A9D2535F1C7907784E426394D4AA276FC2A13A6E1090657D0DE0471073E3F8CB4EE6A616046E5C55A0CDB5459178EB78C1D8C8972A5822E4274AF3346941039F7C90B7188360B9FFCD0E94EE22282CA48904FD4AA06835B33308F5AF673B"
    ];

    const BIG_COMPOSITE: [&str;4] = [
        "CE8E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867",
        "5FFB4E665707B0D9C5D3856B9B67D4751425AEB6575F97F697E446856FFCF159105FECE66D2CDE9DEA958966FE67A0D51ECDFC0FCAD3EACA293485FA2FBCC9DF3B055DE51F14B82EA39D3331C6E6B753C331E06DC8F1F0558EFF0D7F928C0EA6961DD02CFC898ECAE9BFA18919F5113B702964B06E58987CEFFEE05F4BBE4CA3F3D702F528B5540D92947F781B12D67E7A4AE1D5AEAF8BB703789C1574B52381908496060E0150CB55A6D1069B02DA73952E7E8B67C9C0E41A89F5E8C5452510DFCADC3276D26010A2C1F4CD18C07BD2B0F8CEA28DE21AA73D1426E3F5862D02EE2C42B636E4679D2BDA16C336C2FA29E8DEC663088BFDB035205785077BB6B01E3D183E05C42A1AAEAC1B3BA635D8911C704C033C15243DDCC44570EDAA6F651FF61BA698664D391698292C2834E9095B17EB3AC38819BE50BA08F417FBF3F3DBAA7A64F9D0E24D50AF0685074D82D17544010B68295BC07340B46519B184E9E0C01513C57E78E07C7D19C0E0A2ED0432449110DCB0766B6A30B2F02BDAAF75",
        "BEDCDE2405B8A18D6C7615FCFF97DB1C29CD2CA69F1BB1432E690E1E947836FC1DE9160D5C2ADEE52ED244F7997ECCE19FF979D00CC3CCE3784DA6C6495D0D87337B24ABB0FD848C79EBBCF298349396FAE4031A3B7EC2BF313CAEF36AB191CAD36D4AEFDFFA87F72DAACB2EA854FFFCCC66E99C2896911EBA93341C006DD3AA4DD06B432B2D3FCD79B5F7C61DED181B734B2DC1C869E498B2647E8C4301DBFD1787F1C7F5E687D118F2A5D410DB73689586377AA9273DEEC051B60DB813DD0C22FAD561BABE3C59CC67EB284387EE6D3F8C38F6A0B34DE82CEF929B853C3B1A52C6CD6B87AA0A882C30F8B716B3687CCB8EB9EC1BF67407C5142315D2BDFFA5D37E0ADB968593BC66A999695DF11B0164B21A62F7A0A7006D49EF8DEB31408E66AD53A4A6BE38F20EF09C84C729A9544EDF854274DC2120CAFA1BC08E20E7C7F1969DCD4C2C08DCB8AB419B6A8B22F1D6F183B1912E54B045C84E95E668D282073EF9216E3106C173FF9A1D29DC445059491209FA9540D06B666611EB5ECE77",
        "5F6E6F2A02DC50C6B63B0AFE7FCBED8E14E696534F8DD8A19734870F4A3C1B7E0EF48B06AE156F729769227BCCBF6670CFFCBCE80661E671BC26D36324AE86C399BD9255D87EC2463CF5DE794C1A49CB7D72018D1DBF615F989E5779B558C8E569B6A577EFFD43FB96D56597542A7FFE663374CE144B488F5D499A0E0036E9D526E835A195969FE6BCDAFBE30EF68C0DB9A596E0E434F24C59323F462180EDFE8BC3F8E3FAF343E88C7952EA086DB9B44AC31BBD54939EF76028DB06DC09EE86117D6AB0DD5F1E2CE633F59421C3F7369FC61C7B5059A6F41677C94DC29E1D8D296366B5C3D5054416187C5B8B59B43E65C75CF60DFB3A03E28A118AE95EFFD2E9BF056DCB42C9DE3354CCB4AEF88D80B2590D317BD0538036A4F7C6F598A0473356A9D2535F1C7907784E426394D4AA276FC2A13A6E1090657D0DE0471073E3F8CB4EE6A616046E5C55A0CDB5459178EB78C1D8C8972A5822E4274AF3346941039F7C90B7188360B9FFCD0E94EE22282CA48904FD4AA06835B33308F5AF673B"
    ];

    #[test]
    fn test_3072_prime() {
        for p_str in BIG_PRIMES {
            let p = Integer::from_str_radix(p_str, 16).unwrap();
            assert!(miller_rabin(&p, K));
        }
    }

    #[test]
    fn test_3072_composite() {
        for p_str in BIG_COMPOSITE {
            let p = Integer::from_str_radix(p_str, 16).unwrap();
            assert!(!miller_rabin(&p, K));
        }
    }

    #[test]
    fn test_safe_prime() {
        let p =  Integer::from(Integer::parse_radix(
            "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867", 16
        ).unwrap());
        assert!(miller_rabin(&p, K));
        assert!(miller_rabin_safe(&p, K));
    }

    #[test]
    fn test_performance() {
        let p =  Integer::from(Integer::parse_radix(
            "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867", 16
        ).unwrap());
        let begin_rug = SystemTime::now();
        let res_rug = is_prime(&p, 100);
        let duration_rug = begin_rug.elapsed().unwrap();
        let begin_gmpmee = SystemTime::now();
        let res_gmpmee = miller_rabin(&p, 100);
        let duration_gmpmee = begin_gmpmee.elapsed().unwrap();
        assert!(res_rug);
        assert!(res_gmpmee);
        /*assert!(
            duration_rug > duration_gmpmee,
            "The duration of spown (={} ms) is bigger than duration with rug (={} ms)",
            duration_gmpmee.as_millis(),
            duration_rug.as_millis()
        );*/
        println!("Duration rug: {} ms", duration_rug.as_millis());
        println!("Duration gmpmee: {} ms", duration_gmpmee.as_millis());
    }
}
