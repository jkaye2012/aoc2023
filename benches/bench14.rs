use criterion::{criterion_group, criterion_main, Criterion};

fn day14(c: &mut Criterion) {
    let input = include_str!("../input/2023/day14.txt").trim();
    let mut group = c.benchmark_group("day14");
    let gen = aoc2023::day14::generate(input);
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day14::generate(input));
    });
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day14::dish_tilt(input));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day14::dish_tilts(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day14::generate(input);
            aoc2023::day14::dish_tilt(input);
            aoc2023::day14::dish_tilts(&gen);
        })
    });
}

criterion_group!(benches, day14);
criterion_main!(benches);
