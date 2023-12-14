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

type Elem = (usize, usize, char);

#[derive(Clone)]
pub struct SparseMatrix {
    elems: Vec<Elem>,
    rows: usize,
    cols: usize,
    next_pos: Vec<usize>,
}

impl SparseMatrix {
    pub fn north_load(&self) -> usize {
        self.elems
            .iter()
            .filter(|(_, _, c)| *c != '#')
            .map(|(y, _, _)| self.rows - y)
            .sum()
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn tilt_north(&mut self) {
        self.elems
            .sort_by(|(y, x, _), (yb, xb, _)| (x, y).cmp(&(xb, yb)));
        self.next_pos.fill(0);
        for (y, x, c) in &mut self.elems {
            if *c == '#' {
                self.next_pos[*x] = *y + 1;
            } else {
                *y = self.next_pos[*x];
                self.next_pos[*x] += 1;
            }
        }
    }

    pub fn tilt_west(&mut self) {
        self.elems
            .sort_by(|(y, x, _), (yb, xb, _)| (y, x).cmp(&(yb, xb)));
        self.next_pos.fill(0);
        for (y, x, c) in &mut self.elems {
            if *c == '#' {
                self.next_pos[*y] = *x + 1;
            } else {
                *x = self.next_pos[*y];
                self.next_pos[*y] += 1;
            }
        }
    }

    pub fn tilt_south(&mut self) {
        self.elems
            .sort_by(|(y, x, _), (yb, xb, _)| (xb, yb).cmp(&(x, y)));
        self.next_pos.fill(self.rows - 1);
        for (y, x, c) in &mut self.elems {
            if *c == '#' {
                self.next_pos[*x] = *y - 1;
            } else {
                *y = self.next_pos[*x];
                self.next_pos[*x] -= 1;
            }
        }
    }

    pub fn tilt_east(&mut self) {
        self.elems
            .sort_by(|(y, x, _), (yb, xb, _)| (yb, xb).cmp(&(y, x)));
        self.next_pos.fill(self.cols - 1);
        for (y, x, c) in &mut self.elems {
            if *c == '#' {
                self.next_pos[*y] = *x - 1;
            } else {
                *x = self.next_pos[*y];
                self.next_pos[*y] -= 1;
            }
        }
    }
}

#[aoc_generator(day14, part2)]
pub fn generate(input: &str) -> SparseMatrix {
    let rows = input.lines().count();
    let cols = input.find('\n').unwrap();
    let elems = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .filter(|(_, c)| *c != '.')
        .map(|(idx, c)| (idx / cols, idx % cols, c))
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
        matrix.cycle();
        let n = matrix.north_load();
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
