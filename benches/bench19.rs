use criterion::{criterion_group, criterion_main, Criterion};

fn day19(c: &mut Criterion) {
    let input = include_str!("../input/2023/day19.txt").trim();
    let mut group = c.benchmark_group("day19");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day19::generate(input));
    });
    let gen = aoc2023::day19::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day19::accepted(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day19::all_accepted(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day19::generate(input);
            aoc2023::day19::accepted(&gen);
            aoc2023::day19::all_accepted(&gen);
        })
    });
}

criterion_group!(benches, day19);
criterion_main!(benches);
