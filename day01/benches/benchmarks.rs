use criterion::{criterion_group, criterion_main, Criterion};

use tasks::task1;
use tasks::task2;

fn task1_benchmark(c: &mut Criterion) {
    let content = include_str!("../input/challenge.txt");
    c.bench_function("challenge task 1", |b| b.iter(|| task1::solve(content)));
}

fn task2_benchmark(c: &mut Criterion) {
    let content = include_str!("../input/challenge.txt");
    c.bench_function("challenge task 2", |b| b.iter(|| task2::solve(content)));
}

criterion_group!(benches, task1_benchmark, task2_benchmark);
criterion_main!(benches);
