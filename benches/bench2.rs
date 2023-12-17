use criterion::{criterion_group, criterion_main, Criterion};

fn day2(c: &mut Criterion) {
    let input = include_str!("../input/2023/day2.txt").trim();
    let mut group = c.benchmark_group("day2");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day2::generate(input));
    });
    let gen = aoc2023::day2::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day2::cube_game(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day2::cube_power(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day2::generate(input);
            aoc2023::day2::cube_game(&gen);
            aoc2023::day2::cube_power(&gen);
        })
    });
}

criterion_group!(benches, day2);
criterion_main!(benches);
