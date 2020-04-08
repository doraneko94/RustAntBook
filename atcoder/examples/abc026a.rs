pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let a = read1::<usize>();
    println!("{}", (a / 2) * ((a + 2 - 1) / 2));
}