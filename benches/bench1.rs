use criterion::{criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    let input: &str = include_str!("../input/2023/day1.txt").trim();
    let mut group = c.benchmark_group("day1");
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day1::trebuchet_simple(input))
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day1::trebuchet_wordy(input));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            aoc2023::day1::trebuchet_simple(input);
            aoc2023::day1::trebuchet_wordy(input);
        });
    });
}

criterion_group!(benches, day1);
criterion_main!(benches);
