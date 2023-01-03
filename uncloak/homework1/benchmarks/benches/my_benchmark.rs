use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use benchmarks::RSA;



fn benchmark_rsa(c: &mut Criterion) {
    let data = b"stringtoencrypt";
    let rsa = RSA::new();

    c.bench_function(
        "encrypt",
        |f| f.iter(|| rsa.encrypt(data))
    );
}

criterion_group!(benches, benchmark_rsa);
criterion_main!(benches);
