use criterion::{criterion_group, criterion_main, Criterion};

fn day6(c: &mut Criterion) {
    let input = include_str!("../input/2023/day6.txt").trim();
    let mut group = c.benchmark_group("day6");
    group.bench_function("generator1", |b| {
        b.iter(|| aoc2023::day6::generate(input));
    });
    group.bench_function("generator2", |b| {
        b.iter(|| aoc2023::day6::generate_kerning(input));
    });
    let gen = aoc2023::day6::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day6::race(&gen));
    });
    let gen2 = aoc2023::day6::generate_kerning(input);
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day6::race_kerning(&gen2));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day6::generate(input);
            aoc2023::day6::race(&gen);
            let gen2 = aoc2023::day6::generate_kerning(input);
            aoc2023::day6::race_kerning(&gen2);
        })
    });
}

criterion_group!(benches, day6);
criterion_main!(benches);
