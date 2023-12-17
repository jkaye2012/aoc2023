use std::fmt::Display;

#[inline(always)]
fn num_wins(winning: &str, ours: &str) -> u32 {
    let mut won = [false; 100];
    winning
        .split(' ')
        .filter(|w| !w.is_empty())
        .for_each(|w| won[w.parse::<usize>().unwrap()] = true);
    ours.split(' ')
        .filter(|o| !o.is_empty() && won[o.parse::<usize>().unwrap()])
        .count()
        .try_into()
        .unwrap()
}

#[aoc(day4, part1)]
pub fn scratchcard(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.split_once(" | ").unwrap());
    lines
        .map(|(winning, ours)| {
            let wins = num_wins(winning, ours);
            if wins > 0 {
                2u32.pow(wins - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn scratchcards(input: &str) -> usize {
    let num = input.lines().count();
    let lines = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.split_once(" | ").unwrap());
    let mut results = vec![0u32; num];
    let mut n = num - 1;
    for (winning, ours) in lines.rev() {
        let wins = num_wins(winning, ours);
        results[n] = wins;
        for i in 0..wins {
            results[n] += results[n + i as usize + 1];
        }
        n -= 1;
    }
    results.iter().sum::<u32>() as usize + num
}

struct Res(u32, usize);

impl Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)?;
        Ok(())
    }
}

fn _scratchcards_combined(input: &str) -> Res {
    let num = input.lines().count();
    let lines = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.split_once(" | ").unwrap());
    let mut results = vec![0u32; num];
    let mut n = num - 1;
    let mut score = 0;
    for (winning, ours) in lines.rev() {
        let wins = num_wins(winning, ours);
        if wins > 0 {
            score += 2u32.pow(wins - 1);
        }
        results[n] = wins;
        for i in 0..wins {
            results[n] += results[n + i as usize + 1];
        }
        n -= 1;
    }
    Res(score, results.iter().sum::<u32>() as usize + num)
}
