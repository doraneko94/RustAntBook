fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut s = read1::<String>().chars().collect::<Vec<char>>();
    s.insert(4, ' ');
    println!("{}", s.iter().map(|&c| c).collect::<String>());
}