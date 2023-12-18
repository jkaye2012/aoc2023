#[aoc_generator(day18, part1)]
pub fn generate1(input: &str) -> Vec<(isize, isize, isize)> {
    let mut prev = (0isize, 0isize);
    input
        .lines()
        .rev()
        .map(|line| {
            let mut it = line.split(' ');
            let dir = it.next().unwrap();
            let mag = it.next().unwrap().parse::<isize>().unwrap();
            let (y, x) = match dir {
                "R" => (prev.0, prev.1 + mag),
                "D" => (prev.0 + mag, prev.1),
                "L" => (prev.0, prev.1 - mag),
                "U" => (prev.0 - mag, prev.1),
                _ => panic!("Invalid direction"),
            };

            prev = (y, x);
            (y, x, mag)
        })
        .collect()
}

#[aoc_generator(day18, part2)]
pub fn generate2(input: &str) -> Vec<(isize, isize, isize)> {
    let mut prev = (0isize, 0isize);
    input
        .lines()
        .rev()
        .map(|line| {
            let it = line.split(' ');
            let hex = u32::from_str_radix(&it.skip(2).next().unwrap()[2..8], 16).unwrap();
            let dir = hex & 0xF;
            let mag = (hex >> 4) as isize;
            let (y, x) = match dir {
                0 => (prev.0, prev.1 + mag),
                1 => (prev.0 + mag, prev.1),
                2 => (prev.0, prev.1 - mag),
                3 => (prev.0 - mag, prev.1),
                _ => panic!("Invalid direction"),
            };

            prev = (y, x);
            (y, x, mag)
        })
        .collect()
}

#[aoc(day18, part1)]
#[aoc(day18, part2)]
pub fn lava_trench_area(input: &[(isize, isize, isize)]) -> isize {
    let mut prev = (0isize, 0isize);
    let mut boundary = 0isize;
    let mut interior = 0;
    for (y, x, mag) in input {
        boundary += mag;

        interior += (prev.0 + y) * (prev.1 - x);
        prev = (*y, *x);
    }
    interior += prev.0 * -prev.1;
    interior /= 2;
    interior -= 1;
    boundary / 2 + interior.abs()
}
