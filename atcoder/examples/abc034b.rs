fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    if n % 2 == 0 {
        println!("{}", n - 1);
    } else {
        println!("{}", n + 1);
    }
}