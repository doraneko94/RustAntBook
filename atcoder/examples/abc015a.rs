pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let a = read1::<String>();
    let b = read1::<String>();

    if a.len() > b.len() {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}