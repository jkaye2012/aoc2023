fn main() {
    let input = include_str!("../input/2023/day17.txt").trim();
    let city = aoc2023::day17::generate(input);
    println!("{}", aoc2023::day17::minimize_heat_loss_ultra(&city));
}
