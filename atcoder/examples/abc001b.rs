pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let m = read1::<usize>();

    if m < 100 {
        println!("00");
    } else if m <= 5000 {
        let km = m * 10 / 1000;
        println!("{:02}", km);
    } else if m <= 30000 {
        let km = m / 1000;
        println!("{}", km + 50);
    } else if m <= 70000 {
        let km = m / 1000;
        println!("{}", (km - 30) / 5 + 80)
    } else {
        println!("89");
    }
}