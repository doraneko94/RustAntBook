pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let a = read1::<usize>();
    let mut b = 1000 - a;
    let mut ans = 0;
    for &c in [500, 100, 50, 10, 5, 1].iter() {
        ans += b / c;
        b %= c;
    }
    println!("{}", ans);
}