pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let mut ans = 1000;

    for _ in 0..n {
        let t = read1::<usize>();
        ans = std::cmp::min(ans, t);
    }

    println!("{}", ans);
}