#[aoc(day5, part1)]
fn almanac(input: &str) -> isize {
    let mut lines = input.lines();
    let mut seeds = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| (s.parse::<isize>().unwrap(), false))
        .collect::<Vec<(isize, bool)>>();

    let mut range = [0isize; 3];
    for line in lines.skip(2) {
        if line.ends_with(':') {
            continue;
        }
        if line.is_empty() {
            for seed in seeds.iter_mut() {
                seed.1 = false;
            }
            continue;
        }

        line.split(' ')
            .map(|s| s.parse::<isize>().unwrap())
            .enumerate()
            .for_each(|(idx, val)| range[idx] = val);
        let src_range = range[1]..range[1] + range[2];
        let transform = range[0] - range[1];

        for seed in seeds.iter_mut() {
            if seed.1 {
                continue;
            }
            if src_range.contains(&seed.0) {
                seed.0 += transform;
                seed.1 = true;
            }
        }
    }

    return *seeds.iter().map(|(s, _)| s).min().unwrap();
}

#[derive(Debug)]
struct SeedMap {
    src_start: isize,
    src_end: isize,
    transform: isize,
}

impl SeedMap {
    pub fn new(raw: &[isize; 3]) -> Self {
        SeedMap {
            src_start: raw[1],
            src_end: raw[1] + raw[2] - 1,
            transform: raw[0] - raw[1],
        }
    }

    pub fn overlap(&self, range: &SeedRange) -> Option<(SeedRange, SeedRange, Option<SeedRange>)> {
        if self.src_start <= range.start && self.src_end >= range.end {
            Some((
                SeedRange::invalid(),
                SeedRange {
                    start: range.start + self.transform,
                    end: range.end + self.transform,
                    valid: true,
                },
                None,
            ))
        } else if self.src_start <= range.start && self.src_end >= range.start {
            Some((
                SeedRange {
                    start: self.src_end + 1,
                    end: range.end,
                    valid: true,
                },
                SeedRange {
                    start: range.start + self.transform,
                    end: self.src_end + self.transform,
                    valid: true,
                },
                None,
            ))
        } else if self.src_start <= range.end && self.src_end >= range.end {
            Some((
                SeedRange {
                    start: range.start,
                    end: self.src_start - 1,
                    valid: true,
                },
                SeedRange {
                    start: self.src_start + self.transform,
                    end: range.end + self.transform,
                    valid: true,
                },
                None,
            ))
        } else if self.src_start > range.start && self.src_end < range.end {
            Some((
                SeedRange {
                    start: range.start,
                    end: self.src_start - 1,
                    valid: true,
                },
                SeedRange {
                    start: self.src_start + self.transform,
                    end: self.src_end + self.transform,
                    valid: true,
                },
                Some(SeedRange {
                    start: self.src_end + 1,
                    end: range.end,
                    valid: true,
                }),
            ))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct SeedRange {
    start: isize,
    end: isize,
    valid: bool,
}

impl SeedRange {
    pub fn invalid() -> Self {
        SeedRange {
            start: 0,
            end: 0,
            valid: false,
        }
    }
}

#[aoc(day5, part2)]
fn almanac_ranged(input: &str) -> isize {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let mut seed_ranges = Vec::new();
    for i in 0..seeds.len() / 2 {
        seed_ranges.push(SeedRange {
            start: seeds[2 * i],
            end: seeds[2 * i] + seeds[2 * i + 1] - 1,
            valid: true,
        })
    }

    let mut hit_seed_ranges = Vec::new();
    let mut pending_seed_ranges = Vec::new();
    let mut range = [0isize; 3];
    for line in lines.skip(2) {
        if line.ends_with(':') {
            continue;
        }
        if line.is_empty() {
            for seed in hit_seed_ranges.drain(0..) {
                seed_ranges.push(seed);
            }
            continue;
        }

        line.split(' ')
            .map(|s| s.parse::<isize>().unwrap())
            .enumerate()
            .for_each(|(idx, val)| range[idx] = val);
        let seed_map = SeedMap::new(&range);

        for seed_range in seed_ranges.iter_mut().filter(|s| s.valid) {
            if let Some((missed, hit, more)) = seed_map.overlap(&seed_range) {
                seed_range.start = missed.start;
                seed_range.end = missed.end;
                seed_range.valid = missed.valid;
                hit_seed_ranges.push(hit);
                if let Some(range) = more {
                    pending_seed_ranges.push(range);
                }
            }
        }
        for seed in pending_seed_ranges.drain(0..) {
            seed_ranges.push(seed);
        }
    }

    for seed in hit_seed_ranges.drain(0..) {
        seed_ranges.push(seed);
    }

    return seed_ranges
        .iter()
        .filter(|s| s.valid)
        .map(|r| r.start)
        .min()
        .unwrap();
}
