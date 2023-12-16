use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn all(c: &mut Criterion) {
    let input15: &str = include_str!("../input/2023/day15.txt").trim();
    let input16: &str = include_str!("../input/2023/day16.txt").trim();
    let mut group = c.benchmark_group("2023");
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.bench_function("all", |b| {
        b.iter(|| {
            aoc2023::day15::hash_sum(input15);
            aoc2023::day15::focusing_power(input15);
            let gen = aoc2023::day16::generate(input16);
            aoc2023::day16::mirror_energy(black_box(&gen));
            aoc2023::day16::max_mirror_energy(black_box(&gen));
        })
    });
}

criterion_group!(benches, all);
criterion_main!(benches);
