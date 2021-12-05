use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use advent2021_lib::day01;
use advent2021_lib::get_input;

pub fn benchmark(c: &mut Criterion) {
    let depths = day01::get_data(&get_input(1)).unwrap();
    let mut group = c.benchmark_group("day1::main");
    for n in [2, 3, 4].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| day01::depths_increasing(&depths, n))
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
