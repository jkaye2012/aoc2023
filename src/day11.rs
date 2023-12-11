struct Universe {
    galaxies: Vec<bool>,
    row_len: usize,
    x_adj: Vec<usize>,
    y_adj: Vec<usize>,
}

#[aoc_generator(day11)]
fn generate(input: &str) -> Universe {
    let galaxies: Vec<bool> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c == '#')
        .collect();
    let row_len = input.find('\n').unwrap();
    let col_len = galaxies.len() / row_len;

    let mut x_adj = vec![0; row_len];
    let mut curr_x_adj = 0;
    for idx in 0..row_len {
        if !galaxies.iter().skip(idx).step_by(row_len).any(|b| *b) {
            curr_x_adj += 1;
        }
        x_adj[idx] = curr_x_adj;
    }

    let mut curr_y_adj = 0;
    let mut y_adj = vec![0; col_len];
    for idx in (0..galaxies.len()).step_by(row_len) {
        if !galaxies[idx..idx + row_len].iter().any(|b| *b) {
            curr_y_adj += 1;
        }
        y_adj[idx / row_len] = curr_y_adj;
    }

    Universe {
        galaxies,
        row_len,
        x_adj,
        y_adj,
    }
}

fn total_galaxy_distance(universe: &Universe, age_multiplier: usize) -> usize {
    let mut result = 0;
    let mut points: Vec<(usize, usize)> = Vec::new();
    for (idx, _) in universe.galaxies.iter().enumerate().filter(|(_, b)| **b) {
        let (mut y, mut x) = (idx / universe.row_len, idx % universe.row_len);
        y += universe.y_adj[y] * age_multiplier - universe.y_adj[y];
        x += universe.x_adj[x] * age_multiplier - universe.x_adj[x];
        for (yb, xb) in &points {
            result += yb.abs_diff(y) + xb.abs_diff(x);
        }
        points.push((y, x));
    }
    result
}

#[aoc(day11, part1)]
fn young_galaxy_distance(universe: &Universe) -> usize {
    total_galaxy_distance(universe, 2)
}

#[aoc(day11, part2)]
fn old_galaxy_distance(universe: &Universe) -> usize {
    total_galaxy_distance(universe, 1000000)
}
