use criterion::{criterion_group, criterion_main, Criterion};

fn day4(c: &mut Criterion) {
    let input = include_str!("../input/2023/day4.txt").trim();
    let mut group = c.benchmark_group("day4");
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day4::scratchcard(&input));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day4::scratchcards(&input));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            aoc2023::day4::scratchcard(&input);
            aoc2023::day4::scratchcards(&input);
        })
    });
}

criterion_group!(benches, day4);
criterion_main!(benches);
