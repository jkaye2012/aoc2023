use criterion::{criterion_group, criterion_main, Criterion};

fn day9(c: &mut Criterion) {
    let input = include_str!("../input/2023/day9.txt").trim();
    let mut group = c.benchmark_group("day9");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day9::generate(input));
    });
    let gen = aoc2023::day9::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day9::sensor_next(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day9::sensor_prev(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day9::generate(input);
            aoc2023::day9::sensor_next(&gen);
            aoc2023::day9::sensor_prev(&gen);
        })
    });
}

criterion_group!(benches, day9);
criterion_main!(benches);
