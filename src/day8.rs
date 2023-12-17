use std::collections::HashMap;

use crate::util::lcm;

#[derive(Debug)]
pub struct Map {
    directions: Vec<usize>,
    nodes: Vec<[usize; 2]>,
    start_indices: Vec<usize>,
    end_indices: Vec<usize>,
}

#[aoc_generator(day8)]
pub fn generate(input: &str) -> Map {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|d| (d == 'R') as usize)
        .collect();

    let mut node_names: Vec<&str> = lines
        .skip(1)
        .map(|l| l.split_once(" =").unwrap().0)
        .collect();
    node_names.sort();

    let mut start_indices = Vec::new();
    let mut end_indices = Vec::new();
    let mut idx_by_node: HashMap<&str, usize> = HashMap::new();
    node_names.iter().enumerate().for_each(|(i, n)| {
        if n.ends_with('A') {
            start_indices.push(i);
        } else if n.ends_with('Z') {
            end_indices.push(i);
        }
        idx_by_node.insert(n, i);
    });

    let mut nodes = vec![[0, 0]; node_names.len()];
    input.lines().skip(2).for_each(|l| {
        let (from, rest) = l.split_once(" = (").unwrap();
        let (l, rr) = rest.split_once(", ").unwrap();
        let r = &rr[0..3];
        nodes[idx_by_node[from]] = [idx_by_node[l], idx_by_node[r]];
    });

    Map {
        directions,
        nodes,
        start_indices,
        end_indices,
    }
}

fn steps(map: &Map, start: usize, end: Option<usize>) -> usize {
    let num_directions = map.directions.len();
    let mut count = 0;
    let mut idx = start;
    if let Some(end_idx) = end {
        while idx != end_idx {
            let dir = count % num_directions;
            idx = map.nodes[idx][map.directions[dir]];
            count += 1;
        }
    } else {
        while !map.end_indices.contains(&idx) {
            let dir = count % num_directions;
            idx = map.nodes[idx][map.directions[dir]];
            count += 1;
        }
    }

    count
}

#[aoc(day8, part1)]
pub fn camel_map(map: &Map) -> usize {
    steps(map, 0, Some(map.nodes.len() - 1))
}

#[aoc(day8, part2)]
pub fn ghost_map(map: &Map) -> usize {
    std::thread::scope(|s| {
        let mut handles = Vec::new();
        for start in &map.start_indices {
            handles.push(s.spawn(|| steps(map, *start, None)));
        }
        let mut result = 1;
        for handle in handles {
            result = lcm(result, handle.join().unwrap());
        }
        result
    })
}
