#[derive(Default)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Game {
    rounds: Vec<Round>,
}

impl Game {
    pub fn maxes(&self) -> Round {
        let mut result = Round::default();
        for round in &self.rounds {
            result.red = std::cmp::max(result.red, round.red);
            result.green = std::cmp::max(result.green, round.green);
            result.blue = std::cmp::max(result.blue, round.blue);
        }
        result
    }
}

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| Game {
            rounds: line
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split("; ")
                .map(|round| {
                    let mut res = Round::default();
                    for color in round.split(", ") {
                        let mut it = color.split(" ");
                        let num = u32::from_str_radix(it.next().unwrap(), 10).unwrap();
                        match it.next().unwrap() {
                            "red" => res.red = num,
                            "blue" => res.blue = num,
                            "green" => res.green = num,
                            _ => panic!(),
                        }
                    }
                    res
                })
                .collect(),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn cube_game(games: &[Game]) -> u32 {
    let mut game_idx = 1;
    let mut result = 0;
    for game in games {
        let maxes = game.maxes();
        if maxes.red <= 12 && maxes.green <= 13 && maxes.blue <= 14 {
            result += game_idx;
        }
        game_idx += 1;
    }
    result
}

#[aoc(day2, part2)]
pub fn cube_power(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| {
            let maxes = g.maxes();
            maxes.red * maxes.green * maxes.blue
        })
        .sum()
}
