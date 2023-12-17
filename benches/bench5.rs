use criterion::{criterion_group, criterion_main, Criterion};

fn day5(c: &mut Criterion) {
    let input = include_str!("../input/2023/day5.txt").trim();
    let mut group = c.benchmark_group("day5");
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day5::almanac(&input));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day5::almanac_ranged(&input));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            aoc2023::day5::almanac(&input);
            aoc2023::day5::almanac_ranged(&input);
        })
    });
}

criterion_group!(benches, day5);
criterion_main!(benches);
