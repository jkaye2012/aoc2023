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
