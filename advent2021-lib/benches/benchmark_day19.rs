use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent2021_lib::day19;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("day19::benchmarks::map", |b| {
        b.iter(|| {
            let report = black_box(day19::parse(day19::DAY.example).unwrap());
            report.map()
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
