#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use to_kana::*;

fn a_hira_benchmark(c: &mut Criterion) {
    c.bench_function(
        "a hira",
        |b| b.iter(|| hira(black_box("a"))),
    );
}

criterion_group!(benches, a_hira_benchmark);
criterion_main!(benches);
