use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Tile {
    visits: u8,
    len: usize,
    class: char,
}

#[derive(Clone, Debug)]
pub struct TrailMap {
    map: Vec<Tile>,
    rows: usize,
    cols: usize,
    start: usize,
    end: usize,
}

#[aoc_generator(day23)]
pub fn generate(input: &str) -> TrailMap {
    let cols = input.find('\n').unwrap();
    let rows = input.lines().count();
    let start = input.find('.').unwrap();
    let end = input.rfind('.').unwrap() - rows + 1;
    let mut map = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| Tile {
            visits: 0,
            len: 0,
            class: c,
        })
        .collect::<Vec<Tile>>();

    for row in 0..rows {
        for col in 0..cols {
            let idx = row * cols + col;
            let tile = &map[idx];
            if tile.class == '#' {
                continue;
            } else if tile.class != '.' {
                map[idx].visits = 1;
            } else {
                let mut visits = 0;
                if row > 0 && map[(row - 1) * cols + col].class == 'v' {
                    visits += 1;
                }
                if col > 0 && map[row * cols + col - 1].class == '>' {
                    visits += 1;
                }
                if row < rows - 1 && map[(row + 1) * cols + col].class == '^' {
                    visits += 1;
                }
                if col < cols - 1 && map[row * cols + col + 1].class == '<' {
                    visits += 1;
                }

                map[idx].visits = visits.max(1);
            }
        }
    }

    TrailMap {
        map,
        rows,
        cols,
        start,
        end,
    }
}

#[aoc(day23, part1)]
pub fn longest_walk(trail_map: &TrailMap) -> usize {
    let mut tm = trail_map.clone();
    tm.map[trail_map.start].visits = 0;
    tm.map[trail_map.start].len = 1;

    let mut to_explore = VecDeque::new();
    to_explore.push_front(tm.start + tm.cols);
    while let Some(exp) = to_explore.pop_back() {
        tm.map[exp].visits -= 1;
        if tm.map[exp].visits > 0 {
            continue;
        }

        let row = exp / tm.cols;
        let col = exp % tm.cols;

        if row > 0 {
            let idx = (row - 1) * tm.cols + col;
            if tm.map[idx].visits > 0 {
                to_explore.push_back(idx);
            } else {
                tm.map[exp].len = tm.map[exp].len.max(tm.map[idx].len);
            }
        }
        if col > 0 {
            let idx = row * tm.cols + col - 1;
            if tm.map[idx].visits > 0 {
                to_explore.push_back(idx);
            } else {
                tm.map[exp].len = tm.map[exp].len.max(tm.map[idx].len);
            }
        }
        if row < tm.rows - 1 {
            let idx = (row + 1) * tm.cols + col;
            if tm.map[idx].visits > 0 {
                to_explore.push_back(idx);
            } else {
                tm.map[exp].len = tm.map[exp].len.max(tm.map[idx].len);
            }
        }
        if col < tm.cols - 1 {
            let idx = row * tm.cols + col + 1;
            if tm.map[idx].visits > 0 {
                to_explore.push_back(idx);
            } else {
                tm.map[exp].len = tm.map[exp].len.max(tm.map[idx].len);
            }
        }

        tm.map[exp].len += 1;
    }

    // for idx in 0..tm.map.len() {
    //     if idx % tm.cols == 0 {
    //         println!()
    //     }
    //     print!("{:#02} ", tm.map[idx].len)
    // }
    // println!();

    tm.map[tm.end].len - 1
}

fn brute_path(
    trail_map: &TrailMap,
    idx: usize,
    curr_len: usize,
    visited: &mut [bool],
    dist: &mut [usize],
) {
    if visited[idx] {
        return;
    }
    visited[idx] = true;
    if dist[idx] < curr_len {
        dist[idx] = curr_len;
    }
    let row = idx / trail_map.cols;
    let col = idx % trail_map.cols;
    let map = &trail_map.map;

    let up = (row - 1) * trail_map.cols + col;
    if row > 0 && map[up].class != '#' {
        brute_path(trail_map, up, curr_len + 1, visited, dist);
    }

    let right = row * trail_map.cols + col - 1;
    if col > 0 && map[right].class != '#' {
        brute_path(trail_map, right, curr_len + 1, visited, dist);
    }

    let down = (row + 1) * trail_map.cols + col;
    if row < trail_map.rows - 1 && map[down].class != '#' {
        brute_path(trail_map, down, curr_len + 1, visited, dist);
    }

    let left = row * trail_map.cols + col + 1;
    if col < trail_map.cols - 1 && map[left].class != '#' {
        brute_path(trail_map, left, curr_len + 1, visited, dist);
    }

    visited[idx] = false;
}

#[aoc(day23, part2)]
pub fn longest_dry_walk(input: &TrailMap) -> usize {
    let mut visited = vec![false; input.map.len()];
    let mut dist = vec![0; input.map.len()];
    brute_path(&input, input.start, 0, &mut visited, &mut dist);

    dist[input.end]
}
