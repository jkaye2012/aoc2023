use std::fmt::Display;

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn _lcm_multiple(numbers: &[usize]) -> usize {
    let first = numbers[0];
    numbers.iter().skip(1).fold(first, |l, &x| lcm(x, l))
}

pub struct Res<A, B>(pub A, pub B);

impl<A, B> Display for Res<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)?;
        Ok(())
    }
}
