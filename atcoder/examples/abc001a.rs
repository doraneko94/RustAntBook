pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let h1 = read1::<isize>();
    let h2 = read1::<isize>();

    println!("{}", h1 - h2);
}