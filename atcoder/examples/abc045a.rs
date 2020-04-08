fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let a = read1::<usize>();
    let b = read1::<usize>();
    let h = read1::<usize>();

    println!("{}", (a + b) * h / 2);
}