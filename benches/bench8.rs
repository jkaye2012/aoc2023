use criterion::{criterion_group, criterion_main, Criterion};

fn day8(c: &mut Criterion) {
    let input = include_str!("../input/2023/day8.txt").trim();
    let mut group = c.benchmark_group("day8");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day8::generate(input));
    });
    let gen = aoc2023::day8::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day8::camel_map(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day8::ghost_map(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day8::generate(input);
            aoc2023::day8::camel_map(&gen);
            aoc2023::day8::ghost_map(&gen);
        })
    });
}

criterion_group!(benches, day8);
criterion_main!(benches);
