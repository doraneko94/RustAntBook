pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let a = read1::<String>();
    if a == "a".to_string() {
        println!("-1");
    } else {
        println!("a");
    }
}