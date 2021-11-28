use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent2020::day13;

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day13::find_time");
    let busses: Vec<day13::Bus> = "7,13,x,x,59,x,31,19"
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    group.bench_function("find_time_loop", |b| {
        b.iter(|| day13::find_time_loop(black_box(&busses), black_box(0)))
    });
    group.bench_function("find_time_loop_max", |b| {
        b.iter(|| day13::find_time_loop_max(black_box(&busses), black_box(0)))
    });
    group.bench_function("find_time_thread", |b| {
        b.iter(|| day13::find_time_thread(black_box(&busses), black_box(0)))
    });
    group.bench_function("find_time_chinese_remaindern", |b| {
        b.iter(|| day13::chinese_remainder_busses(black_box(&busses), black_box(0)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
