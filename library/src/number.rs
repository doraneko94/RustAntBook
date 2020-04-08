pub fn gcd(a: usize, b: usize) -> usize {
    let (a, b) = if a < b {
        (b, a)
    } else {
        (a, b)
    };
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}