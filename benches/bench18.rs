use criterion::{criterion_group, criterion_main, Criterion};

fn day18(c: &mut Criterion) {
    let input = include_str!("../input/2023/day18.txt").trim();
    let mut group = c.benchmark_group("day18");
    group.bench_function("generator1", |b| {
        b.iter(|| aoc2023::day18::generate1(input));
    });
    group.bench_function("generator2", |b| {
        b.iter(|| aoc2023::day18::generate2(input));
    });
    let gen = aoc2023::day18::generate1(input);
    let gen2 = aoc2023::day18::generate2(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day18::lava_trench_area(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day18::lava_trench_area(&gen2));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day18::generate1(input);
            let gen2 = aoc2023::day18::generate2(input);
            aoc2023::day18::lava_trench_area(&gen);
            aoc2023::day18::lava_trench_area(&gen2);
        })
    });
}

criterion_group!(benches, day18);
criterion_main!(benches);
