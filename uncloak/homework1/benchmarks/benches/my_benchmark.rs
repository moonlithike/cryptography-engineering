use benchmarks::RSA;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn benchmark_rsa(c: &mut Criterion) {
    let data = b"stringtoencrypt";

    c.bench_function("genereate keypair", |f| f.iter(|| RSA::new()));

    let rsa = RSA::new();
    c.bench_function("encrypt", |f| f.iter(|| rsa.encrypt(data)));

    let rsa = RSA::new();
    let enc_data = rsa.encrypt(b"string");
    c.bench_function("decrypt", |f| f.iter(|| rsa.decrypt(&enc_data)));
}

criterion_group!(benches, benchmark_rsa);
criterion_main!(benches);
