pub const ARITY: usize = 21;
pub type SensorValue = [i64; ARITY];

#[aoc_generator(day9)]
pub fn generate(input: &str) -> Vec<SensorValue> {
    input
        .lines()
        .map(|l| {
            let mut value = [0i64; ARITY];
            l.split(' ')
                .map(|i| i.parse::<i64>().unwrap())
                .enumerate()
                .for_each(|(idx, val)| value[idx] = val);
            value
        })
        .collect()
}

fn trends(readings: &SensorValue) -> (i64, i64) {
    let mut last = [0i64; ARITY];
    let mut curr = [0i64; ARITY];
    last[0] = readings[ARITY - 1];
    curr[0] = readings[ARITY - 1];
    let mut idx = 0;
    while idx != ARITY - 1 {
        idx += 1;
        let mut prev_diff = curr[0] - readings[ARITY - (1 + idx)];
        curr[0] = readings[ARITY - (1 + idx)];
        for update_idx in 1..idx {
            let prev = prev_diff;
            prev_diff = curr[update_idx] - prev_diff;
            curr[update_idx] = prev;
        }
        last[idx] = prev_diff;
        curr[idx] = prev_diff;
    }
    let mut prev = 0;
    for i in (1..ARITY - 1).rev() {
        prev = curr[i - 1] - prev;
    }
    (prev, last.iter().sum())
}

#[aoc(day9, part1)]
pub fn sensor_next(input: &[SensorValue]) -> i64 {
    input.iter().map(|sv| trends(sv).1).sum()
}

#[aoc(day9, part2)]
pub fn sensor_prev(input: &[SensorValue]) -> i64 {
    input.iter().map(|sv| trends(sv).0).sum()
}
