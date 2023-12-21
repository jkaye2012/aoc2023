use criterion::{criterion_group, criterion_main, Criterion};

fn day20(c: &mut Criterion) {
    let input = include_str!("../input/2023/day20.txt").trim();
    let mut group = c.benchmark_group("day20");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day20::generate(input));
    });
    let gen = aoc2023::day20::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day20::button_1000(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day20::generate(input);
            aoc2023::day20::button_1000(&gen);
        })
    });
}

criterion_group!(benches, day20);
criterion_main!(benches);
