fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
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

fn main() {
    let a = read1::<usize>();
    let b = read1::<usize>();
    let n = read1::<usize>();
    let lcm = a * b / gcd(a, b);

    println!("{}", (n + lcm - 1) / lcm * lcm);
}