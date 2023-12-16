use std::collections::HashMap;

#[aoc(day14 part1)]
pub fn dish_tilt(input: &str) -> usize {
    let rows = input.lines().count();
    let cols = input.find('\n').unwrap();
    let mut curr_scores = vec![rows; cols];

    let mut col = 0;
    let mut row = rows;
    let mut result = 0;
    for c in input.chars() {
        match c {
            '\n' => {
                col = 0;
                row -= 1;
                continue;
            }
            '#' => {
                curr_scores[col] = row - 1;
            }
            'O' => {
                result += curr_scores[col];
                curr_scores[col] -= 1;
            }
            '.' => {}
            _ => panic!("Bad char"),
        }

        col += 1;
    }

    result
}

#[derive(Clone)]
pub struct SparseMatrix {
    elems: Vec<u8>,
    rows: usize,
    cols: usize,
    next_pos: Vec<usize>,
}

impl SparseMatrix {
    pub fn cycle(&mut self) -> usize {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east()
    }

    pub fn tilt_north(&mut self) {
        self.next_pos.fill(0);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = col + row * self.cols;
                let elem = self.elems[idx];
                if elem == b'#' {
                    self.next_pos[col] = row + 1;
                } else if elem == b'O' {
                    self.elems.swap(idx, self.next_pos[col] * self.cols + col);
                    self.next_pos[col] += 1;
                }
            }
        }
    }

    pub fn tilt_west(&mut self) {
        self.next_pos.fill(0);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = col + row * self.cols;
                let elem = self.elems[idx];
                if elem == b'#' {
                    self.next_pos[row] = col + 1;
                } else if elem == b'O' {
                    self.elems.swap(idx, row * self.cols + self.next_pos[row]);
                    self.next_pos[row] += 1;
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        self.next_pos.fill(self.rows - 1);
        for row in (0..self.rows).rev() {
            for col in 0..self.cols {
                let idx = col + row * self.cols;
                let elem = self.elems[idx];
                if elem == b'#' {
                    self.next_pos[col] = row - 1;
                } else if elem == b'O' {
                    self.elems.swap(idx, self.next_pos[col] * self.cols + col);
                    self.next_pos[col] -= 1;
                }
            }
        }
    }

    pub fn tilt_east(&mut self) -> usize {
        let mut load = 0;
        self.next_pos.fill(self.cols - 1);
        for row in 0..self.rows {
            for col in (0..self.cols).rev() {
                let idx = col + row * self.cols;
                let elem = self.elems[idx];
                if elem == b'#' {
                    self.next_pos[row] = col - 1;
                } else if elem == b'O' {
                    load += self.rows - row;
                    self.elems.swap(idx, row * self.cols + self.next_pos[row]);
                    self.next_pos[row] -= 1;
                }
            }
        }
        load
    }
}

#[aoc_generator(day14, part2)]
pub fn generate(input: &str) -> SparseMatrix {
    let rows = input.lines().count();
    let cols = input.find('\n').unwrap();
    let elems = input
        .as_bytes()
        .iter()
        .filter(|b| **b != b'\n')
        .map(|b| *b)
        .collect();
    SparseMatrix {
        elems,
        rows,
        cols,
        next_pos: vec![0; cols],
    }
}

fn try_find_cycle(
    cache: &mut HashMap<usize, usize>,
    idx: usize,
    val: usize,
) -> Option<(usize, usize)> {
    if let Some(prev) = cache.get_mut(&val) {
        let len = idx - *prev;
        if len > 2 && idx % len == 0 {
            return Some((*prev, idx - *prev));
        }
        *cache.get_mut(&val).unwrap() = idx;
    } else {
        cache.insert(val, idx);
    }
    None
}

#[aoc(day14, part2)]
pub fn dish_tilts(input: &SparseMatrix) -> usize {
    let mut matrix = input.clone();
    let mut recur = Vec::new();
    let mut cache = HashMap::new();
    let mut cycle;
    let mut idx = 0;
    loop {
        let n = matrix.cycle();
        recur.push(n);
        cycle = try_find_cycle(&mut cache, idx, n);
        if cycle.is_some() {
            break;
        }
        idx += 1;
    }

    let (idx, len) = cycle.unwrap();
    let view = &recur[idx..idx + len];
    view[(1000000000 - idx) % len - 1]
}
