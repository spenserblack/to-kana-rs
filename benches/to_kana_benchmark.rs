#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use to_kana::*;

fn hira_benchmark(c: &mut Criterion) {
    c.bench_function(
        "a hira",
        |b| b.iter(|| hira(black_box("a"))),
    );
    c.bench_function(
        "aiueo hira",
        |b| b.iter(|| hira(black_box("aiueo"))),
    );
    c.bench_function(
        "konnichiha hira",
        |b| b.iter(|| hira(black_box("konnichiha"))),
    );
}

fn kata_benchmark(c: &mut Criterion) {
    c.bench_function(
        "a kata",
        |b| b.iter(|| kata(black_box("a"))),
    );
    c.bench_function(
        "aiueo kata",
        |b| b.iter(|| kata(black_box("aiueo"))),
    );
    c.bench_function(
        "konnichiha kata",
        |b| b.iter(|| kata(black_box("konnichiha"))),
    );
}

criterion_group!(benches, hira_benchmark, kata_benchmark);
criterion_main!(benches);
