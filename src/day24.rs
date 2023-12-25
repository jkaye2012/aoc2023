#[derive(Debug)]
pub struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3d {
    pub fn new(s: &str) -> Self {
        let mut it = s.split(", ");
        Self {
            x: it.next().unwrap().trim().parse().unwrap(),
            y: it.next().unwrap().trim().parse().unwrap(),
            z: it.next().unwrap().trim().parse().unwrap(),
        }
    }

    pub fn add(&self, p: &Point3d) -> Self {
        Self {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        }
    }

    pub fn sub(&self, p: &Point3d) -> Self {
        Self {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        }
    }

    pub fn manhattan_distance_2d(&self, p: (f64, f64)) -> f64 {
        (self.x - p.0).abs() + (self.y - p.1).abs()
    }
}

#[derive(Debug)]
pub struct Line {
    point: Point3d,
    velocity: Point3d,
    next: Point3d,
    prev: Point3d,
}

impl Line {
    pub fn new(s: &str) -> Self {
        let (ps, vs) = s.split_once(" @ ").unwrap();
        let point = Point3d::new(ps);
        let velocity = Point3d::new(vs);
        let next = point.add(&velocity);
        let prev = point.sub(&velocity);
        Self {
            point,
            velocity,
            next,
            prev,
        }
    }

    fn in_future(&self, p: (f64, f64)) -> bool {
        let dx = p.0 - self.point.x;
        let dy = p.1 - self.point.y;

        dx * self.velocity.x + dy * self.velocity.y > 0.0
    }

    pub fn intersects(&self, other: &Line, area_min: f64, area_max: f64) -> bool {
        let determinant = (self.next.x - self.point.x) * (other.next.y - other.point.y)
            - (self.next.y - self.point.y) * (other.next.x - other.point.x);
        if determinant.abs() < 1e-9 {
            //println!("Parallel");
            return false;
        }

        let px = ((self.next.x * self.point.y - self.next.y * self.point.x)
            * (other.next.x - other.point.x)
            - (self.next.x - self.point.x)
                * (other.next.x * other.point.y - other.next.y * other.point.x))
            / determinant;
        let py = ((self.next.x * self.point.y - self.next.y * self.point.x)
            * (other.next.y - other.point.y)
            - (self.next.y - self.point.y)
                * (other.next.x * other.point.y - other.next.y * other.point.x))
            / determinant;
        //println!("Intersected at {:?}", (px, py));

        if px < area_min || px > area_max || py < area_min || py > area_max {
            //println!("Outside of test area");
            return false;
        }

        let future_a = self.in_future((px, py));
        let future_b = other.in_future((px, py));
        //println!("{:?}, {:?}", self, other);
        // if !future_a && !future_b {
        //     println!("Both segments in the past");
        // } else if !future_a {
        //     println!("Segment A in the past");
        // } else if !future_b {
        //     println!("Segment B in the past");
        // }

        future_a && future_b
    }
}

#[aoc_generator(day24)]
pub fn generate(input: &str) -> Vec<Line> {
    input.lines().map(Line::new).collect()
}

#[aoc(day24, part1)]
pub fn intersections(lines: &[Line]) -> usize {
    let mut intersected = 0;
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            //println!("Testing {} {}", i, j);
            //if lines[i].intersects(&lines[j], 7.0, 27.0) {
            if lines[i].intersects(&lines[j], 200000000000000.0, 400000000000000.0) {
                //println!("Success");
                intersected += 1;
            }
        }
    }
    intersected
}
