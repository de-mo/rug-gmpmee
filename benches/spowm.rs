use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rug::{rand::RandState, Integer};
use rug_gmpmee::spown::spowm;

pub fn rug_spown(bases: &[Integer], exponents: &[Integer], modulus: &Integer) -> Integer {
    bases
        .iter()
        .zip(exponents.iter())
        .map(|(b, e)| Integer::from(b.pow_mod_ref(e, modulus).unwrap()))
        .fold(Integer::ONE.clone(), |acc, v| (acc * v) % modulus)
}

fn bench_spowns(c: &mut Criterion) {
    let mut group = c.benchmark_group("spown");
    let p =  Integer::from(Integer::parse_radix(
        "CE9E0307D2AE75BDBEEC3E0A6E71A279417B56C955C602FFFD067586BACFDAC3BCC49A49EB4D126F5E9255E57C14F3E09492B6496EC8AC1366FC4BB7F678573FA2767E6547FA727FC0E631AA6F155195C035AF7273F31DFAE1166D1805C8522E95F9AF9CE33239BF3B68111141C20026673A6C8B9AD5FA8372ED716799FE05C0BB6EAF9FCA1590BD9644DBEFAA77BA01FD1C0D4F2D53BAAE965B1786EC55961A8E2D3E4FE8505914A408D50E6B99B71CDA78D8F9AF1A662512F8C4C3A9E72AC72D40AE5D4A0E6571135CBBAAE08C7A2AA0892F664549FA7EEC81BA912743F3E584AC2B2092243C4A17EC98DF079D8EECB8B885E6BBAFA452AAFA8CB8C08024EFF28DE4AF4AC710DCD3D66FD88212101BCB412BCA775F94A2DCE18B1A6452D4CF818B6D099D4505E0040C57AE1F3E84F2F8E07A69C0024C05ACE05666A6B63B0695904478487E78CD0704C14461F24636D7A3F267A654EEDCF8789C7F627C72B4CBD54EED6531C0E54E325D6F09CB648AE9185A7BDA6553E40B125C78E5EAA867", 16
    ).unwrap());
    let mut rand = RandState::new();

    let len = 20;
    let mut bases = vec![];
    (0..len).for_each(|_| bases.push(Integer::from(Integer::random_bits(3072, &mut rand))));
    let mut exponents = vec![];
    (0..len).for_each(|_| exponents.push(Integer::from(Integer::random_bits(3072, &mut rand))));

    group.bench_with_input(BenchmarkId::new("rug", &len), &len, |b, _| {
        b.iter(|| rug_spown(&bases, &exponents, &p))
    });
    group.bench_with_input(BenchmarkId::new("gmpmee", &len), &len, |b, _| {
        b.iter(|| spowm(&bases, &exponents, &p).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench_spowns);
criterion_main!(benches);
