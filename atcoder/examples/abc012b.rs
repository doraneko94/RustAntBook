pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let h = n / 3600;
    let m = n % 3600 / 60;
    let s = n % 60;

    println!("{:02}:{:02}:{:02}", h, m, s);
}