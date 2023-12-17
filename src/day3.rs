#[derive(Clone)]
pub struct Num {
    val: u32,
    range: Range,
}

pub enum Hit {
    Missed,
    Hit,
    ImpossiblePre,
    ImpossiblePost,
}

#[derive(Copy, Clone)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn overlaps(&self, other: Self) -> bool {
        other.start <= self.end && self.start <= other.end
    }

    pub fn from_idx(start: usize) -> Self {
        Self {
            start,
            end: start + 2,
        }
    }
}

impl Num {
    pub fn hit(&self, pre: Range, inl: Range, post: Range) -> Hit {
        if self.range.end < pre.start {
            Hit::ImpossiblePre
        } else if self.range.start > post.end {
            Hit::ImpossiblePost
        } else if self.range.overlaps(pre) || self.range.overlaps(inl) || self.range.overlaps(post)
        {
            Hit::Hit
        } else {
            Hit::Missed
        }
    }
}

pub struct Schematic {
    line_len: usize,
    symbols: Vec<(usize, char)>,
    nums: Vec<Num>,
}

#[aoc_generator(day3)]
pub fn generate(input: &str) -> Schematic {
    let mut line_len = None;
    let mut symbols = Vec::new();
    let mut nums = Vec::new();
    let mut idx = 0usize;
    let mut curr_num: Option<Num> = None;

    for ch in input.chars() {
        if ch == '\n' {
            if let None = line_len {
                line_len = Some(idx);
            }
            if let Some(num) = curr_num {
                nums.push(num);
                curr_num = None;
            }
            continue;
        }
        if let Some(digit) = ch.to_digit(10) {
            if let Some(ref mut num) = curr_num {
                num.val *= 10;
                num.val += digit;
                num.range.end = idx;
            } else {
                curr_num = Some(Num {
                    val: digit,
                    range: Range {
                        start: idx,
                        end: idx,
                    },
                })
            }
            idx += 1;
            continue;
        }

        if let Some(num) = curr_num {
            nums.push(num);
            curr_num = None;
        }
        if ch != '.' {
            symbols.push((idx, ch));
        }

        idx += 1;
    }

    Schematic {
        line_len: line_len.unwrap(),
        symbols,
        nums,
    }
}

#[aoc(day3, part1)]
pub fn schematic(input: &Schematic) -> u32 {
    let mut result = 0;
    let mut nums = input.nums.clone();
    let mut to_remove = Vec::new();
    for (symbol_idx, _) in input.symbols.iter() {
        let pre = Range::from_idx(symbol_idx - input.line_len - 1);
        let inl = Range::from_idx(symbol_idx - 1);
        let post = Range::from_idx(symbol_idx + input.line_len - 1);

        for (idx, num) in nums.iter().enumerate() {
            match num.hit(pre, inl, post) {
                Hit::Hit => {
                    result += num.val;
                    to_remove.push(idx);
                    continue;
                }
                Hit::ImpossiblePre => {
                    to_remove.push(idx);
                    continue;
                }
                Hit::ImpossiblePost => {
                    break;
                }
                _ => {}
            }
        }
        for idx in to_remove.drain(0..).rev() {
            nums.remove(idx);
        }
    }
    result
}

#[aoc(day3, part2)]
pub fn schematic_gears(input: &Schematic) -> u32 {
    let mut result = 0;
    let mut nums = input.nums.clone();
    let mut to_remove = Vec::new();
    for (symbol_idx, _) in input.symbols.iter() {
        let pre = Range::from_idx(symbol_idx - input.line_len - 1);
        let inl = Range::from_idx(symbol_idx - 1);
        let post = Range::from_idx(symbol_idx + input.line_len - 1);
        let mut hits = 0;
        let mut ratio = 1;

        for (idx, num) in nums.iter().enumerate() {
            match num.hit(pre, inl, post) {
                Hit::Hit => {
                    hits += 1;
                    ratio *= num.val;
                    to_remove.push(idx);
                    continue;
                }
                Hit::ImpossiblePre => {
                    to_remove.push(idx);
                    continue;
                }
                Hit::ImpossiblePost => {
                    break;
                }
                _ => {}
            }
        }
        if hits == 2 {
            result += ratio;
        }
        for idx in to_remove.drain(0..).rev() {
            nums.remove(idx);
        }
    }
    result
}
