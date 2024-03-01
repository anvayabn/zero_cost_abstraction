use criterion::{criterion_group, criterion_main, Criterion};


use zero_cost_abs_bm::with_abstraction;
use zero_cost_abs_bm::no_abstraction;

fn benchmark_functions(c: &mut Criterion) {
    let vec = vec![1, 2, 3, 4, 5, 6];

    c.bench_function("No_abstraction", |b| b.iter(|| no_abstraction(vec.clone())));
    c.bench_function("With_abstraction", |b| b.iter(|| with_abstraction(vec.clone())));
}

criterion_group!(benches, benchmark_functions);
criterion_main!(benches);