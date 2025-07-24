use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("Just some static text that should be v v v v v v v v fast")]
struct Static;

fn static_text() -> String {
    Static.to_string()
}

#[derive(Oxiplate)]
#[oxiplate = "external.html.oxip"]
struct Data<'a> {
    title: &'a str,
    message: &'a str,
}

fn variables(title: &str, message: &str) -> String {
    Data { title, message }.to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut fast_group = c.benchmark_group("fast-tests");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    fast_group.sample_size(10_000);

    fast_group.bench_function("static", |b| b.iter(static_text));

    fast_group.bench_function("variables", |b| {
        b.iter(|| variables(black_box("hello"), black_box("world")));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}
criterion_main!(benches);
