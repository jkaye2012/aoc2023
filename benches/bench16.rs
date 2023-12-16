use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day16(c: &mut Criterion) {
    let input: &str = include_str!("../input/2023/day16.txt").trim();
    let mut group = c.benchmark_group("day16");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day16::generate(black_box(input)))
    });
    let gen = aoc2023::day16::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day16::mirror_energy(black_box(&gen)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day16::max_mirror_energy(black_box(&gen)))
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day16::generate(input);
            aoc2023::day16::mirror_energy(black_box(&gen));
            aoc2023::day16::max_mirror_energy(black_box(&gen));
        })
    });
}

criterion_group!(benches, day16);
criterion_main!(benches);
