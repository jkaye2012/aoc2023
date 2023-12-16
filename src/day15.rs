#[aoc(day15, part1)]
pub fn hash_sum(input: &str) -> usize {
    let mut result = 0usize;
    let mut curr = 0usize;
    for b in input.as_bytes() {
        if *b == b',' {
            result += curr;
            curr = 0;
            continue;
        }

        curr += *b as usize;
        curr *= 17;
        curr %= 256;
    }
    result + curr
}

struct Lens<'a> {
    label: &'a [u8],
    focal_length: u8,
}

struct Lenses<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> Lenses<'a> {
    pub fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    pub fn process(&mut self, bytes: &'a [u8], start_idx: usize, end_idx: usize) {
        let last = bytes[end_idx - 1];
        if last == b'-' {
            let label = &bytes[start_idx..end_idx - 1];
            let mut to_remove = None;
            for (idx, lens) in self.lenses.iter().enumerate() {
                if lens.label == label {
                    to_remove = Some(idx);
                    break;
                }
            }
            if let Some(idx) = to_remove {
                self.lenses.remove(idx);
            }
        } else {
            let label = &bytes[start_idx..end_idx - 2];
            for lens in &mut self.lenses {
                if lens.label == label {
                    lens.focal_length = last;
                    return;
                }
            }
            self.lenses.push(Lens {
                label,
                focal_length: last,
            });
        }
    }

    pub fn focal_length(&self, box_multiplier: usize) -> usize {
        let mut result = 0;
        let mut slot_num = 1;
        for lens in &self.lenses {
            result += box_multiplier * slot_num * (lens.focal_length - b'0') as usize;
            slot_num += 1;
        }
        result
    }
}

#[aoc(day15, part2)]
pub fn focusing_power(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut boxes: [Lenses; 256] = std::array::from_fn(|_| Lenses::new());
    let mut start = 0usize;
    let mut hash = 0usize;
    for (idx, b) in input.as_bytes().iter().enumerate() {
        if *b == b',' {
            boxes[hash].process(bytes, start, idx);
            hash = 0;
            start = idx + 1;
            continue;
        }

        if *b >= b'a' && *b <= b'z' {
            hash += *b as usize;
            hash *= 17;
            hash %= 256;
        }
    }
    boxes[hash].process(bytes, start, input.len());
    let mut result = 0;
    for idx in 0..256 {
        result += boxes[idx].focal_length(idx + 1);
    }
    result
}
