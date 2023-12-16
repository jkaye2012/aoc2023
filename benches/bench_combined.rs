use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn all(c: &mut Criterion) {
    let input1 = include_str!("../input/2023/day1.txt").trim();
    let input12 = include_str!("../input/2023/day12.txt").trim();
    let input14 = include_str!("../input/2023/day14.txt").trim();
    let input15 = include_str!("../input/2023/day15.txt").trim();
    let input16 = include_str!("../input/2023/day16.txt").trim();
    let mut group = c.benchmark_group("2023");
    group.bench_function("all", |b| {
        b.iter(|| {
            aoc2023::day1::trebuchet_simple(input1);
            aoc2023::day1::trebuchet_wordy(input1);
            aoc2023::day12::arrangements(input12);
            aoc2023::day12::expanded_arrangements(input12);
            let gen14 = aoc2023::day14::generate(input14);
            aoc2023::day14::dish_tilt(input14);
            aoc2023::day14::dish_tilts(&gen14);
            aoc2023::day15::hash_sum(input15);
            aoc2023::day15::focusing_power(input15);
            let gen16 = aoc2023::day16::generate(input16);
            aoc2023::day16::mirror_energy(black_box(&gen16));
            aoc2023::day16::max_mirror_energy(black_box(&gen16));
        })
    });
}

criterion_group!(benches, all);
criterion_main!(benches);
