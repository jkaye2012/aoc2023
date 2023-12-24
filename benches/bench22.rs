use criterion::{criterion_group, criterion_main, Criterion};

fn day22(c: &mut Criterion) {
    let input = include_str!("../input/2023/day22.txt").trim();
    let mut group = c.benchmark_group("day22");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day22::generate(input));
    });
    let gen = aoc2023::day22::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day22::disintegrate(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day22::fall(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day22::generate(input);
            aoc2023::day22::disintegrate(&gen);
            aoc2023::day22::fall(&gen);
        })
    });
}

criterion_group!(benches, day22);
criterion_main!(benches);
