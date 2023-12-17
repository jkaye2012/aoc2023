use rustc_hash::FxHashMap;
use std::collections::BinaryHeap;

pub struct City {
    heat_loss: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl City {
    pub fn len(&self) -> usize {
        self.heat_loss.len()
    }
}

#[aoc_generator(day17)]
pub fn generate(input: &str) -> City {
    let cols = input.find('\n').unwrap();
    let heat_loss = input
        .as_bytes()
        .iter()
        .filter(|b| **b != b'\n')
        .map(|b| b - b'0')
        .collect::<Vec<u8>>();
    let rows = heat_loss.len() / cols;
    City {
        heat_loss,
        rows,
        cols,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(PartialEq, Eq)]
struct Pending {
    heat: u32,
    step: Step,
}

impl Ord for Pending {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| self.step.cmp(&other.step))
    }
}

impl PartialOrd for Pending {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Traversal<'a> {
    city: &'a City,
    cache: FxHashMap<Step, u32>,
    pending: BinaryHeap<Pending>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Step {
    idx: usize,
    dir: Direction,
    steps: u8,
}

impl Step {
    pub fn try_create(
        city: &City,
        from_row: usize,
        from_col: usize,
        from_dir: Direction,
        steps: u8,
        to_dir: Direction,
    ) -> Option<Self> {
        if from_dir == to_dir && steps == 3 || Self::out_of_bounds(city, from_row, from_col, to_dir)
        {
            None
        } else {
            Self::make_step(city, from_row, from_col, from_dir, to_dir, steps)
        }
    }

    pub fn try_create_ultra(
        city: &City,
        from_row: usize,
        from_col: usize,
        from_dir: Direction,
        steps: u8,
        to_dir: Direction,
    ) -> Option<Self> {
        if from_dir == to_dir && steps == 10
            || from_dir != to_dir && steps < 4
            || Self::out_of_bounds(city, from_row, from_col, to_dir)
        {
            None
        } else {
            Self::make_step(city, from_row, from_col, from_dir, to_dir, steps)
        }
    }

    fn out_of_bounds(city: &City, from_row: usize, from_col: usize, to_dir: Direction) -> bool {
        to_dir == Direction::Up && from_row == 0
            || to_dir == Direction::Left && from_col == 0
            || to_dir == Direction::Down && from_row == city.rows - 1
            || to_dir == Direction::Right && from_col == city.cols - 1
    }

    fn make_step(
        city: &City,
        from_row: usize,
        from_col: usize,
        from_dir: Direction,
        dir: Direction,
        steps: u8,
    ) -> Option<Self> {
        let steps = if from_dir == dir { steps + 1 } else { 1 };
        let idx = match dir {
            Direction::Right => from_row * city.cols + from_col + 1,
            Direction::Down => (from_row + 1) * city.cols + from_col,
            Direction::Left => from_row * city.cols + from_col - 1,
            Direction::Up => (from_row - 1) * city.cols + from_col,
        };
        Some(Self { idx, dir, steps })
    }
}

impl<'a> Traversal<'a> {
    pub fn new(city: &'a City) -> Self {
        Traversal {
            city,
            cache: FxHashMap::default(),
            pending: BinaryHeap::new(),
        }
    }

    pub fn minimize(&mut self, ultra: bool) -> u32 {
        let target_idx = self.city.len() - 1;
        let mut curr = Pending {
            step: Step {
                idx: 0,
                dir: Direction::Right,
                steps: 0,
            },
            heat: 0,
        };
        self.cache.insert(curr.step, 0);
        let down = Pending {
            step: Step {
                idx: 0,
                dir: Direction::Down,
                steps: 0,
            },
            heat: 0,
        };
        self.cache.insert(down.step, 0);
        self.pending.push(down);

        while curr.step.idx != target_idx || (ultra && curr.step.steps < 4) {
            for neighbor in self.neighbors(&curr, ultra) {
                if let Some(step) = neighbor {
                    self.maybe_update_pending(curr.step, step);
                }
            }
            curr = self.pending.pop().unwrap();
        }

        curr.heat
    }

    fn neighbors(&self, curr: &Pending, ultra: bool) -> [Option<Step>; 3] {
        let col = curr.step.idx % self.city.cols;
        let row = curr.step.idx / self.city.cols;
        let s = &curr.step;

        if ultra {
            match s.dir {
                Direction::Right => [
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Right),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Up),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Down),
                ],
                Direction::Down => [
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Down),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Right),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Left),
                ],
                Direction::Left => [
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Left),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Down),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Up),
                ],
                Direction::Up => [
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Up),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Left),
                    Step::try_create_ultra(self.city, row, col, s.dir, s.steps, Direction::Right),
                ],
            }
        } else {
            match s.dir {
                Direction::Right => [
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Right),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Up),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Down),
                ],
                Direction::Down => [
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Down),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Right),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Left),
                ],
                Direction::Left => [
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Left),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Down),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Up),
                ],
                Direction::Up => [
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Up),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Left),
                    Step::try_create(self.city, row, col, s.dir, s.steps, Direction::Right),
                ],
            }
        }
    }

    fn maybe_update_pending(&mut self, curr: Step, next: Step) {
        let curr_heat = self.cache.get(&curr).unwrap();
        let heat = curr_heat + self.city.heat_loss[next.idx] as u32;
        let next_cache = self.cache.get(&next);
        let next_heat = next_cache.unwrap_or(&u32::MAX);

        if heat < *next_heat {
            if next_cache.is_none() {
                self.cache.insert(next, heat);
            } else {
                *self.cache.get_mut(&next).unwrap() = heat;
            }
            let pending = Pending { step: next, heat };
            self.pending.push(pending);
        }
    }
}

#[aoc(day17, part1)]
pub fn minimize_heat_loss(city: &City) -> u32 {
    let mut traversal = Traversal::new(city);
    traversal.minimize(false)
}

#[aoc(day17, part2)]
pub fn minimize_heat_loss_ultra(city: &City) -> u32 {
    let mut traversal = Traversal::new(city);
    traversal.minimize(true)
}
