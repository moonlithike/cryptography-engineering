use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand;

fn encrypt_rsa(data: [u8]) -> () {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    pub_key
        .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..])
}

fn benchmark_rsa_with_input(c : &mut Criterion) {
    let input: [u8] = "blahblah";

    c.bench_with_input(
        BenchmarkId::new("input_example", input), &input, |b, &s| {
            b.iter(|| encrypt_rsa(input));
        });
}


criterion_group!(benches, benchmark_rsa_with_input);
criterion_main!(benches);