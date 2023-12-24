use std::{collections::BTreeSet, str::Split};

use rayon::prelude::*;

#[derive(Clone, Debug)]
enum Brick {
    X((usize, usize), usize, usize),
    Y(usize, (usize, usize), usize),
    Z(usize, usize, (usize, usize)),
}

type Coord = (usize, usize, usize);

// max_z, idx
type State = (usize, usize);

impl Brick {
    pub fn new(a: Coord, b: Coord) -> Self {
        let (ax, ay, az) = a;
        let (bx, by, bz) = b;

        if ax != bx {
            Self::X((ax.min(bx), ax.max(bx)), ay, az)
        } else if ay != by {
            Self::Y(ax, (ay.min(by), ay.max(by)), az)
        } else {
            Self::Z(ax, ay, (az.min(bz), az.max(bz)))
        }
    }

    pub fn max_x(&self) -> usize {
        match self {
            Self::X(x, _, _) => x.1,
            Self::Y(x, _, _) => *x,
            Self::Z(x, _, _) => *x,
        }
    }

    pub fn max_y(&self) -> usize {
        match self {
            Self::X(_, y, _) => *y,
            Self::Y(_, y, _) => y.1,
            Self::Z(_, y, _) => *y,
        }
    }

    pub fn min_z(&self) -> usize {
        match self {
            Self::X(_, _, z) => *z,
            Self::Y(_, _, z) => *z,
            Self::Z(_, _, z) => z.0,
        }
    }

    pub fn max_z(&self) -> usize {
        match self {
            Self::X(_, _, z) => *z,
            Self::Y(_, _, z) => *z,
            Self::Z(_, _, z) => z.1,
        }
    }

    pub fn height(&self) -> usize {
        self.max_z() - self.min_z() + 1
    }

    pub fn update(&mut self, myidx: usize, cols: usize, state: &mut [State]) -> BTreeSet<usize> {
        let mut supporting = BTreeSet::new();
        let mut max_z = 0;
        match self {
            Self::X((xa, xb), y, _) => {
                for idx in *y * cols + *xa..=*y * cols + *xb {
                    let (z, prev) = state[idx];
                    if z > max_z {
                        max_z = z;
                        supporting.clear();
                        supporting.insert(prev);
                    } else if z > 0 && z == max_z {
                        supporting.insert(prev);
                    }
                }
                for idx in *y * cols + *xa..=*y * cols + *xb {
                    state[idx] = (max_z + self.height(), myidx)
                }
            }
            Self::Y(x, (ya, yb), _) => {
                for idx in (*ya * cols + *x..=*yb * cols + *x).step_by(cols) {
                    let (z, prev) = state[idx];
                    if z > max_z {
                        max_z = z;
                        supporting.clear();
                        supporting.insert(prev);
                    } else if z > 0 && z == max_z {
                        supporting.insert(prev);
                    }
                }
                for idx in (*ya * cols + *x..=*yb * cols + *x).step_by(cols) {
                    state[idx] = (max_z + self.height(), myidx)
                }
            }
            Self::Z(x, y, _) => {
                let idx = *y * cols + *x;
                let (z, prev) = state[idx];
                if z > max_z {
                    max_z = z;
                    supporting.insert(prev);
                } else if z > 0 && z == max_z {
                    supporting.insert(prev);
                }
                state[idx] = (max_z + self.height(), myidx)
            }
        }

        supporting
    }
}

type ZIndex = (usize, usize);

#[derive(Clone, Debug)]
pub struct Snapshot {
    bricks: Vec<Brick>,
    unprocessed: BTreeSet<ZIndex>,
    cols: usize,
    rows: usize,
}

#[inline(always)]
fn parse(it: &mut Split<'_, char>) -> usize {
    it.next().unwrap().parse::<usize>().unwrap()
}

#[inline(always)]
fn coord(xyz: &str) -> Coord {
    let mut it = xyz.split(',');
    (parse(&mut it), parse(&mut it), parse(&mut it))
}

#[aoc_generator(day22)]
pub fn generate(input: &str) -> Snapshot {
    let mut unprocessed = BTreeSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let bricks = input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            let (a, b) = l.split_once('~').unwrap();
            let brick = Brick::new(coord(a), coord(b));
            unprocessed.insert((brick.min_z(), idx));
            max_x = max_x.max(brick.max_x());
            max_y = max_y.max(brick.max_y());
            brick
        })
        .collect();

    Snapshot {
        bricks,
        unprocessed,
        cols: max_x + 1,
        rows: max_y + 1,
    }
}

fn count_supporting(
    idx: usize,
    fallen: &mut [bool],
    supports: &[Vec<usize>],
    supported_by: &[BTreeSet<usize>],
) -> usize {
    if !fallen[idx] {
        return 0;
    }

    let mut res = 0;
    for s in &supports[idx] {
        if !fallen[*s] && supported_by[*s].iter().all(|s| fallen[*s]) {
            res += 1;
            fallen[*s] = true;
        }
    }

    res + supports[idx]
        .iter()
        .map(|s| count_supporting(*s, fallen, supports, supported_by))
        .sum::<usize>()
}

#[aoc(day22, part1)]
pub fn disintegrate(snapshot: &Snapshot) -> usize {
    let mut snapshot = snapshot.clone();
    let mut state = vec![(0, 0); snapshot.cols * snapshot.rows];
    let mut disintegratable = vec![true; snapshot.bricks.len()];
    while let Some((_, zidx)) = snapshot.unprocessed.pop_first() {
        let brick = &mut snapshot.bricks[zidx];
        let supporting = brick.update(zidx, snapshot.cols, &mut state);
        if supporting.len() == 1 {
            let idx = *supporting.first().unwrap();
            disintegratable[idx] = false;
        }
    }

    disintegratable.iter().filter(|d| **d).count()
}

#[aoc(day22, part2)]
pub fn fall(snapshot: &Snapshot) -> usize {
    let mut snapshot = snapshot.clone();
    let mut state = vec![(0, 0); snapshot.cols * snapshot.rows];
    let mut disintegratable = vec![true; snapshot.bricks.len()];
    let mut supports = vec![vec![]; snapshot.bricks.len()];
    let mut supported_by = vec![BTreeSet::new(); snapshot.bricks.len()];
    while let Some((_, zidx)) = snapshot.unprocessed.pop_first() {
        let brick = &mut snapshot.bricks[zidx];
        let supporting = brick.update(zidx, snapshot.cols, &mut state);
        if supporting.len() == 1 {
            let idx = *supporting.first().unwrap();
            disintegratable[idx] = false;
        }

        for supp in &supporting {
            supports[*supp].push(zidx);
        }
        supported_by[zidx] = supporting;
    }

    disintegratable
        .par_iter()
        .enumerate()
        .filter(|(_, b)| !**b)
        .map(|(d, _)| {
            let mut fallen = vec![false; supports.len()];
            fallen[d] = true;
            count_supporting(d, &mut fallen, &supports, &supported_by)
        })
        .sum()
}
