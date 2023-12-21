use criterion::{criterion_group, criterion_main, Criterion};

fn day21(c: &mut Criterion) {
    let input = include_str!("../input/2023/day21.txt").trim();
    let mut group = c.benchmark_group("day21");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day21::generate(input));
    });
    let gen = aoc2023::day21::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day21::steps_possible(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day21::steps_infinite(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day21::generate(input);
            aoc2023::day21::steps_possible(&gen);
            aoc2023::day21::steps_infinite(&gen);
        })
    });
}

criterion_group!(benches, day21);
criterion_main!(benches);
