use criterion::{criterion_group, criterion_main, Criterion};

fn day15(c: &mut Criterion) {
    let input: &str = include_str!("../input/2023/day15.txt").trim();
    let mut group = c.benchmark_group("day15");
    group.bench_function("part1", |b| b.iter(|| aoc2023::day15::hash_sum(input)));
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day15::focusing_power(input))
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            aoc2023::day15::hash_sum(input);
            aoc2023::day15::focusing_power(input);
        });
    });
}

criterion_group!(benches, day15);
criterion_main!(benches);
