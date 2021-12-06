use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent2021_lib::day06;
use advent2021_lib::get_input;

pub fn benchmark(c: &mut Criterion) {
    let state = day06::parse(&get_input(6)).unwrap();

    c.bench_function("fish_count_array", |b| {
        b.iter(|| day06::fish_count_array(black_box(&state.0), black_box(256)))
    });

    c.bench_function("fish_count_ndarray", |b| {
        b.iter(|| day06::fish_count_array(black_box(&state.0), black_box(256)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
