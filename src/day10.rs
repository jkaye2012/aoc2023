use std::fmt::Display;

// Position update, new direction, winding designator
type Connection = (isize, usize, isize);

struct PipeMap {
    pipes: Vec<isize>,
    // Up, right, down, left
    connections: [[Connection; 7]; 4],
    start: isize,
}

fn char_to_pipe(c: char) -> isize {
    match c {
        '.' => 0,
        '-' => 1,
        '|' => 2,
        'F' => 3,
        'L' => 4,
        'J' => 5,
        '7' => 6,
        'S' => 7,
        _ => panic!("Invalid pipe char"),
    }
}

#[aoc_generator(day10)]
fn generate(input: &str) -> PipeMap {
    let len: isize = input.find('\n').unwrap().try_into().unwrap();
    let mut start = 0;
    let pipes: Vec<isize> = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| {
            if c == 'S' {
                start = i;
            }
            char_to_pipe(c)
        })
        .collect();
    let connections = [
        [
            (0, 0, 0),
            (0, 0, 0),
            (-len, 0, 1),
            (1, 1, 1),
            (0, 0, 0),
            (0, 0, 0),
            (-1, 3, 1),
        ],
        [
            (0, 0, 0),
            (1, 1, 2),
            (0, 0, 0),
            (0, 0, 0),
            (0, 0, 0),
            (-len, 0, 2),
            (len, 2, -1),
        ],
        [
            (0, 0, 0),
            (0, 0, 0),
            (len, 2, -1),
            (0, 0, 0),
            (1, 1, 2),
            (-1, 3, 2),
            (0, 0, 0),
        ],
        [
            (0, 0, 0),
            (-1, 3, 2),
            (0, 0, 0),
            (len, 2, -1),
            (-len, 0, 2),
            (0, 0, 0),
            (0, 0, 0),
        ],
    ];
    PipeMap {
        pipes,
        connections,
        start: start.try_into().unwrap(),
    }
}

struct Res(usize, usize);

impl Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)?;
        Ok(())
    }
}

#[aoc(day10, part1)]
fn furthest_pipe(input: &PipeMap) -> Res {
    let mut count = 1;
    let mut curr_pos = input.start + 1;
    let mut curr_dir = 1;
    let mut winding = vec![0; input.pipes.len()];
    while curr_pos != input.start {
        let (pos, dir, wind) = input.connections[curr_dir][input.pipes[curr_pos as usize] as usize];
        winding[curr_pos as usize] = wind;
        curr_pos += pos;
        curr_dir = dir;
        count += 1;
    }

    winding[curr_pos as usize] = 1;
    let mut enclosed = 0;
    let mut wound = 0;
    for wind in winding {
        if wind == 2 {
            continue;
        } else if wind == 0 {
            if wound != 0 {
                enclosed += 1;
            } else {
            }
        } else {
            wound += wind;
        }
    }

    Res(count / 2, enclosed)
}
