use criterion::{criterion_group, criterion_main, Criterion};

fn day10(c: &mut Criterion) {
    let input = include_str!("../input/2023/day10.txt").trim();
    let mut group = c.benchmark_group("day10");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day10::generate(input));
    });
    let gen = aoc2023::day10::generate(input);
    group.bench_function("part1and2", |b| {
        b.iter(|| aoc2023::day10::furthest_pipe(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day10::generate(input);
            aoc2023::day10::furthest_pipe(&gen);
        })
    });
}

criterion_group!(benches, day10);
criterion_main!(benches);
