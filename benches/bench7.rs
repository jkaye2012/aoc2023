use criterion::{criterion_group, criterion_main, Criterion};

fn day7(c: &mut Criterion) {
    let input = include_str!("../input/2023/day7.txt").trim();
    let mut group = c.benchmark_group("day7");
    group.bench_function("generator1", |b| {
        b.iter(|| aoc2023::day7::generate(input));
    });
    group.bench_function("generator2", |b| {
        b.iter(|| aoc2023::day7::generate_wildcard(input));
    });
    let gen = aoc2023::day7::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day7::total_winnings(&gen));
    });
    let gen2 = aoc2023::day7::generate_wildcard(input);
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day7::total_winnings(&gen2));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day7::generate(input);
            aoc2023::day7::total_winnings(&gen);
            let gen2 = aoc2023::day7::generate_wildcard(input);
            aoc2023::day7::total_winnings(&gen2);
        })
    });
}

criterion_group!(benches, day7);
criterion_main!(benches);
