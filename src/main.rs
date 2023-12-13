fn main() {
    let input = include_str!("../input/2023/day13.txt");
    let mut result = 0;
    for _ in 0..100 {
        let l = aoc2023::day13::generate(input);
        result += aoc2023::day13::reflections_smudged(&l);
    }
    std::process::exit(result.try_into().unwrap());
}
