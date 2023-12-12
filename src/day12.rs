use rustc_hash::FxHashMap;

fn line_arrangements(
    damaged_idx: usize,
    start: usize,
    end: usize,
    conditions: &[char],
    damaged: &[usize],
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if let Some(a) = cache.get(&(damaged_idx, start)) {
        return *a;
    }
    let size = damaged[damaged_idx];
    let last = end - size + 1;
    let mut result = 0;
    for idx in start..last {
        if idx > 0 && conditions[idx - 1] == '#' {
            break;
        } else if idx + size < conditions.len() && conditions[idx + size] == '#'
            || conditions[idx..idx + size].iter().any(|c| *c == '.')
        {
            continue;
        }

        if damaged_idx == damaged.len() - 1 {
            if conditions[idx + size..].iter().any(|c| *c == '#') {
                continue;
            }
            result += 1;
        } else {
            let next = damaged[damaged_idx + 1];
            result += line_arrangements(
                damaged_idx + 1,
                idx + size + 1,
                end + next + 1,
                conditions,
                damaged,
                cache,
            )
        }
    }
    cache.insert((damaged_idx, start), result);
    result
}

#[aoc(day12, part1)]
fn arrangements(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (conditions, damaged_str) = l.split_once(' ').unwrap();
            let damaged = damaged_str
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect::<Vec<usize>>();
            let a = line_arrangements(
                0,
                0,
                conditions.len() - damaged.iter().skip(1).map(|d| d + 1).sum::<usize>(),
                &conditions.chars().collect::<Vec<char>>(),
                &damaged,
                &mut FxHashMap::default(),
            );
            a
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn expanded_arrangements(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (conditions, damaged_str) = l.split_once(' ').unwrap();
            let damaged = damaged_str
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect::<Vec<usize>>();
            let mut d_exp = Vec::new();
            let mut c_exp = Vec::new();
            for i in 0..5 {
                for d in &damaged {
                    d_exp.push(*d);
                }
                for c in conditions.chars() {
                    c_exp.push(c);
                }
                if i != 4 {
                    c_exp.push('?');
                }
            }
            let a = line_arrangements(
                0,
                0,
                c_exp.len() - d_exp.iter().skip(1).map(|d| d + 1).sum::<usize>(),
                &c_exp,
                &d_exp,
                &mut FxHashMap::default(),
            );
            a
        })
        .sum()
}
