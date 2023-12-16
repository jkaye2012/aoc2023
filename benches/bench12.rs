use criterion::{criterion_group, criterion_main, Criterion};

fn day12(c: &mut Criterion) {
    let input = include_str!("../input/2023/day12.txt").trim();
    let mut group = c.benchmark_group("day12");
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day12::arrangements(input));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day12::expanded_arrangements(input));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            aoc2023::day12::arrangements(input);
            aoc2023::day12::expanded_arrangements(input);
        })
    });
}

criterion_group!(benches, day12);
criterion_main!(benches);
