use criterion::{criterion_group, criterion_main, Criterion};

fn day13(c: &mut Criterion) {
    let input = include_str!("../input/2023/day13.txt").trim();
    let mut group = c.benchmark_group("day13");
    group.bench_function("generate", |b| {
        b.iter(|| aoc2023::day13::generate(input));
    });
    let gen = aoc2023::day13::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day13::reflections(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day13::reflections_smudged(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day13::generate(input);
            aoc2023::day13::reflections(&gen);
            aoc2023::day13::reflections_smudged(&gen);
        })
    });
}

criterion_group!(benches, day13);
criterion_main!(benches);
