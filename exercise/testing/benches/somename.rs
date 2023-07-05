use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| sploosh(black_box(-20), black_box(0), black_box(10)))
    });
}

pub fn criterion_benchmark_splish(c: &mut Criterion) {
    c.bench_function("splish 20", |b| {
        b.iter(|| splish(black_box(-20), black_box(0)))
    });
}
criterion_group!(benches, criterion_benchmark_splish);
criterion_main!(benches);
