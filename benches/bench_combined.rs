use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn all(c: &mut Criterion) {
    let input1 = include_str!("../input/2023/day1.txt").trim();
    let input2 = include_str!("../input/2023/day2.txt").trim();
    let input3 = include_str!("../input/2023/day3.txt").trim();
    let input4 = include_str!("../input/2023/day4.txt").trim();
    let input5 = include_str!("../input/2023/day5.txt").trim();
    let input6 = include_str!("../input/2023/day6.txt").trim();
    let input7 = include_str!("../input/2023/day7.txt").trim();
    let input8 = include_str!("../input/2023/day8.txt").trim();
    let input9 = include_str!("../input/2023/day9.txt").trim();
    let input10 = include_str!("../input/2023/day10.txt").trim();
    let input11 = include_str!("../input/2023/day11.txt").trim();
    let input12 = include_str!("../input/2023/day12.txt").trim();
    let input13 = include_str!("../input/2023/day13.txt").trim();
    let input14 = include_str!("../input/2023/day14.txt").trim();
    let input15 = include_str!("../input/2023/day15.txt").trim();
    let input16 = include_str!("../input/2023/day16.txt").trim();
    let input17 = include_str!("../input/2023/day17.txt").trim();
    let input18 = include_str!("../input/2023/day18.txt").trim();
    let mut group = c.benchmark_group("2023");
    group.bench_function("all", |b| {
        b.iter(|| {
            aoc2023::day1::trebuchet_simple(input1);
            aoc2023::day1::trebuchet_wordy(input1);

            let gen2 = aoc2023::day2::generate(input2);
            aoc2023::day2::cube_game(&gen2);
            aoc2023::day2::cube_power(&gen2);

            let gen3 = aoc2023::day3::generate(input3);
            aoc2023::day3::schematic(&gen3);
            aoc2023::day3::schematic_gears(&gen3);

            aoc2023::day4::scratchcard(&input4);
            aoc2023::day4::scratchcards(&input4);

            aoc2023::day5::almanac(&input5);
            aoc2023::day5::almanac_ranged(&input5);

            let gen61 = aoc2023::day6::generate(input6);
            aoc2023::day6::race(&gen61);
            let gen62 = aoc2023::day6::generate_kerning(input6);
            aoc2023::day6::race_kerning(&gen62);

            let gen71 = aoc2023::day7::generate(input7);
            aoc2023::day7::total_winnings(&gen71);
            let gen72 = aoc2023::day7::generate_wildcard(input7);
            aoc2023::day7::total_winnings(&gen72);

            let gen8 = aoc2023::day8::generate(input8);
            aoc2023::day8::camel_map(&gen8);
            aoc2023::day8::ghost_map(&gen8);

            let gen9 = aoc2023::day9::generate(input9);
            aoc2023::day9::sensor_next(&gen9);
            aoc2023::day9::sensor_prev(&gen9);

            let gen10 = aoc2023::day10::generate(input10);
            aoc2023::day10::furthest_pipe(&gen10);

            let gen11 = aoc2023::day11::generate(input11);
            aoc2023::day11::young_galaxy_distance(&gen11);
            aoc2023::day11::old_galaxy_distance(&gen11);

            aoc2023::day12::arrangements(input12);
            aoc2023::day12::expanded_arrangements(input12);

            let gen13 = aoc2023::day13::generate(input13);
            aoc2023::day13::reflections(&gen13);
            aoc2023::day13::reflections_smudged(&gen13);

            let gen14 = aoc2023::day14::generate(input14);
            aoc2023::day14::dish_tilt(input14);
            aoc2023::day14::dish_tilts(&gen14);

            aoc2023::day15::hash_sum(input15);
            aoc2023::day15::focusing_power(input15);

            let gen16 = aoc2023::day16::generate(input16);
            aoc2023::day16::mirror_energy(black_box(&gen16));
            aoc2023::day16::max_mirror_energy(black_box(&gen16));

            let gen17 = aoc2023::day17::generate(input17);
            aoc2023::day17::minimize_heat_loss(&gen17);
            aoc2023::day17::minimize_heat_loss_ultra(&gen17);

            let gen181 = aoc2023::day18::generate1(input18);
            let gen182 = aoc2023::day18::generate2(input18);
            aoc2023::day18::lava_trench_area(&gen181);
            aoc2023::day18::lava_trench_area(&gen182);
        })
    });
}

criterion_group!(benches, all);
criterion_main!(benches);
