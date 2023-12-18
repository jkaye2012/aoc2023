#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn new(s: &str) -> Self {
        match s {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "U" => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn from_hex(hex: u32) -> Self {
        match hex & 0xF {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }
}

type Move = (Direction, u32);

pub struct DigPlan {
    moves: Vec<Move>,
    rows: usize,
    cols: usize,
    start: usize,
}

#[aoc_generator(day18, part1)]
pub fn generate1(input: &str) -> DigPlan {
    let mut moves = Vec::new();
    let mut curr_row = 1i32;
    let mut row_max = 1;
    let mut row_min = 1;
    let mut curr_col = 1i32;
    let mut col_max = 1;
    let mut col_min = 1;
    for line in input.lines() {
        let mut it = line.split(' ');
        let dir = Direction::new(it.next().unwrap());
        let mag = it.next().unwrap().parse::<u8>().unwrap();
        match dir {
            Direction::Right => curr_col += mag as i32,
            Direction::Down => curr_row += mag as i32,
            Direction::Left => curr_col -= mag as i32,
            Direction::Up => curr_row -= mag as i32,
        }
        row_max = row_max.max(curr_row);
        row_min = row_min.min(curr_row);
        col_max = col_max.max(curr_col);
        col_min = col_min.min(curr_col);
        moves.push((dir, mag as u32));
    }

    let rows = row_max - row_min + 1;
    let cols = col_max - col_min + 1;
    let start = (row_min - 1).abs() * cols + (col_min - 1).abs();

    DigPlan {
        moves,
        rows: rows.try_into().unwrap(),
        cols: cols.try_into().unwrap(),
        start: start as usize,
    }
}

// const CORNER_H: u32 = 0x01000000;
// const CORNER_V: u32 = 0x02000000;
// const WALL_H: u32 = 0x04000000;
// const WALL_V: u32 = 0x08000000;

// pub fn lava_trench_size(input: &DigPlan) -> usize {
//     let mut spaces = vec![0; input.rows * input.cols];
//     let mut curr_idx = input.start;
//     for (idx, (dir, mag)) in input.moves.iter().enumerate() {
//         let next_dir = input.moves[(idx + 1) % input.moves.len()].0;
//         match dir {
//             Direction::Right => {
//                 for _ in 1..*mag {
//                     curr_idx += 1;
//                     spaces[curr_idx] = WALL_H;
//                 }
//                 curr_idx += 1;
//                 spaces[curr_idx] = if next_dir == Direction::Down {
//                     CORNER_V
//                 } else {
//                     CORNER_H
//                 };
//             }
//             Direction::Down => {
//                 for _ in 1..*mag {
//                     curr_idx += input.cols;
//                     spaces[curr_idx] = WALL_V;
//                 }
//                 curr_idx += input.cols;
//                 spaces[curr_idx] = if next_dir == Direction::Left {
//                     CORNER_V
//                 } else {
//                     CORNER_H
//                 };
//             }
//             Direction::Left => {
//                 for _ in 1..*mag {
//                     curr_idx -= 1;
//                     spaces[curr_idx] = WALL_H;
//                 }
//                 curr_idx -= 1;
//                 spaces[curr_idx] = if next_dir == Direction::Up {
//                     CORNER_V
//                 } else {
//                     CORNER_H
//                 };
//             }
//             Direction::Up => {
//                 for _ in 1..*mag {
//                     curr_idx -= input.cols;
//                     spaces[curr_idx] = WALL_V;
//                 }
//                 curr_idx -= input.cols;
//                 spaces[curr_idx] = if next_dir == Direction::Right {
//                     CORNER_V
//                 } else {
//                     CORNER_H
//                 };
//             }
//         };
//     }

//     let mut winding = false;
//     let mut area = 0;
//     for (_idx, space) in spaces.iter().enumerate() {
//         // if idx % input.cols == 0 {
//         //     println!();
//         // }
//         if space & (WALL_V | CORNER_V) != 0 {
//             winding = !winding;
//         }
//         if space > &0 {
//             // print!("{}", val);
//             area += 1;
//         } else if winding {
//             //print!("#");
//             area += 1;
//         } else {
//             //print!(".");
//         }
//     }
//     //println!();
//     area
// }

#[aoc_generator(day18, part2)]
pub fn generate2(input: &str) -> DigPlan {
    let mut moves = Vec::new();
    let mut curr_row = 0i32;
    let mut row_max = 0;
    let mut row_min = 0;
    let mut curr_col = 0i32;
    let mut col_max = 0;
    let mut col_min = 0;
    for line in input.lines() {
        let it = line.split(' ');
        let hex = u32::from_str_radix(&it.skip(2).next().unwrap()[2..8], 16).unwrap();
        let dir = Direction::from_hex(hex);
        let mag = hex >> 4;
        match dir {
            Direction::Right => curr_col += mag as i32,
            Direction::Down => curr_row += mag as i32,
            Direction::Left => curr_col -= mag as i32,
            Direction::Up => curr_row -= mag as i32,
        }
        row_max = row_max.max(curr_row);
        row_min = row_min.min(curr_row);
        col_max = col_max.max(curr_col);
        col_min = col_min.min(curr_col);
        moves.push((dir, mag));
    }

    let rows = row_max - row_min + 1;
    let cols = col_max - col_min + 1;
    let start: usize = row_min.abs() as usize * cols as usize + col_min.abs() as usize;

    DigPlan {
        moves,
        rows: rows.try_into().unwrap(),
        cols: cols.try_into().unwrap(),
        start,
    }
}

#[aoc(day18, part1)]
#[aoc(day18, part2)]
pub fn massive_lava_trench(input: &DigPlan) -> usize {
    let mut row_map = vec![Vec::new(); input.rows];
    let mut curr_row = input.start / input.cols;
    let mut curr_col = input.start % input.cols;
    for (idx, (dir, mag)) in input.moves.iter().enumerate() {
        let next_dir = input.moves[(idx + 1) % input.moves.len()].0;
        match dir {
            Direction::Right => {
                curr_col += *mag as usize;
                if next_dir == Direction::Down {
                    row_map[curr_row].push(curr_col);
                }
            }
            Direction::Down => {
                for _ in 1..*mag {
                    curr_row += 1;
                    row_map[curr_row].push(curr_col);
                }
                curr_row += 1;
                if next_dir == Direction::Left {
                    row_map[curr_row].push(curr_col);
                }
            }
            Direction::Left => {
                curr_col -= *mag as usize;
                if next_dir == Direction::Up {
                    row_map[curr_row].push(curr_col);
                }
            }
            Direction::Up => {
                for _ in 1..*mag {
                    curr_row -= 1;
                    row_map[curr_row].push(curr_col);
                }
                curr_row -= 1;
                if next_dir == Direction::Right {
                    row_map[curr_row].push(curr_col);
                }
            }
        };
    }

    let mut total = 0;
    for idx in 0..row_map.len() {
        let row = &mut row_map[idx];
        row.sort();
        for row_idx in 0..row.len() / 2 {
            total += row[row_idx * 2 + 1] - row[row_idx * 2] + 1;
        }
    }
    total
}
