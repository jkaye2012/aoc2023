use criterion::{criterion_group, criterion_main, Criterion};

fn day3(c: &mut Criterion) {
    let input = include_str!("../input/2023/day3.txt").trim();
    let mut group = c.benchmark_group("day3");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day3::generate(input));
    });
    let gen = aoc2023::day3::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day3::schematic(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day3::schematic_gears(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day3::generate(input);
            aoc2023::day3::schematic(&gen);
            aoc2023::day3::schematic_gears(&gen);
        })
    });
}

criterion_group!(benches, day3);
criterion_main!(benches);
