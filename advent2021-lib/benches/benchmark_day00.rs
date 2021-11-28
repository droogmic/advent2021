use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use advent2020::get_string;

use advent2020::day1;

pub fn benchmark(c: &mut Criterion) {
    let expenses = day1::get_data(get_string("day1.txt"));
    let mut group = c.benchmark_group("day1::main");
    for n in [2, 3].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| day1::calc(expenses.to_vec(), n))
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
