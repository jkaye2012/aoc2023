use criterion::{criterion_group, criterion_main, Criterion};

fn day11(c: &mut Criterion) {
    let input = include_str!("../input/2023/day11.txt").trim();
    let mut group = c.benchmark_group("day11");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day11::generate(input));
    });
    let gen = aoc2023::day11::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day11::young_galaxy_distance(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day11::old_galaxy_distance(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day11::generate(input);
            aoc2023::day11::young_galaxy_distance(&gen);
            aoc2023::day11::old_galaxy_distance(&gen);
        })
    });
}

criterion_group!(benches, day11);
criterion_main!(benches);
