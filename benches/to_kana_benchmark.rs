#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use to_kana::*;

fn small_kana_conversion(s: &str) -> to_kana::Result {
    s.small()
}

fn half_width_conversion(s: &str) -> to_kana::Result {
    s.half_width()
}

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

fn small_kana_benchmark(c: &mut Criterion) {
    c.bench_function(
        "や small",
        |b| b.iter(|| small_kana_conversion(black_box("や"))),
    );
    c.bench_function(
        "ヤ small",
        |b| b.iter(|| small_kana_conversion(black_box("ヤ"))),
    );
    c.bench_function(
        "あいうえお small",
        |b| b.iter(|| small_kana_conversion(black_box("あいうえお"))),
    );
    c.bench_function(
        "アイウエオ small",
        |b| b.iter(|| small_kana_conversion(black_box("アイウエオ"))),
    );
}

fn half_width_kana_benchmark(c: &mut Criterion) {
    c.bench_function(
        "ヤ half-width",
        |b| b.iter(|| half_width_conversion(black_box("ヤ"))),
    );
    c.bench_function(
        "アイウエオ half-width",
        |b| b.iter(|| half_width_conversion(black_box("アイウエオ"))),
    );
    c.bench_function(
        "コレハナガイストリング! half-width",
        |b| b.iter(|| half_width_conversion(black_box(
            "コレハナガイストリング!"
        ))),
    );
}

criterion_group!(
    benches,
    hira_benchmark,
    kata_benchmark,
    small_kana_benchmark,
    half_width_kana_benchmark,
);
criterion_main!(benches);
