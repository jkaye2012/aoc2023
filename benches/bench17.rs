use criterion::{criterion_group, criterion_main, Criterion};

fn day17(c: &mut Criterion) {
    let input = include_str!("../input/2023/day17.txt").trim();
    let mut group = c.benchmark_group("day17");
    group.bench_function("generator", |b| {
        b.iter(|| aoc2023::day17::generate(input));
    });
    let gen = aoc2023::day17::generate(input);
    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day17::minimize_heat_loss(&gen));
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day17::minimize_heat_loss_ultra(&gen));
    });
    group.bench_function("combined", |b| {
        b.iter(|| {
            let gen = aoc2023::day17::generate(input);
            aoc2023::day17::minimize_heat_loss(&gen);
            aoc2023::day17::minimize_heat_loss_ultra(&gen);
        })
    });
}

criterion_group!(benches, day17);
criterion_main!(benches);
