type TimeDistancePair = (usize, usize);

#[aoc_generator(day6, part1)]
fn generate(input: &str) -> Vec<TimeDistancePair> {
    let (tline, dline) = input.split_once('\n').unwrap();
    let times = tline
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap());
    let distances = dline
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap());
    times.zip(distances).collect()
}

fn ways_to_win(time: usize, distance: usize) -> usize {
    let mut lo = 0;
    let mut hi = time / 2 + (time & 1);
    while lo != hi - 1 {
        let next = (lo + hi) / 2;
        if next * (time - next) > distance {
            hi = next;
        } else {
            lo = next;
        }
    }
    time - hi * 2 + 1
}

#[aoc(day6, part1)]
fn race(input: &[TimeDistancePair]) -> usize {
    let mut result = 1;
    for (time, distance) in input {
        result *= ways_to_win(*time, *distance);
    }
    result
}

#[aoc_generator(day6, part2)]
fn generate_kerning(input: &str) -> TimeDistancePair {
    let (tline, dline) = input.split_once('\n').unwrap();
    let time = tline
        .chars()
        .skip_while(|c| *c != ' ')
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = dline
        .chars()
        .skip_while(|c| *c != ' ')
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    (time, distance)
}

#[aoc(day6, part2)]
fn race_kerning(input: &TimeDistancePair) -> usize {
    ways_to_win(input.0, input.1)
}
