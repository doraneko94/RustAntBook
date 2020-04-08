fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut s = read1::<String>().chars().collect::<Vec<char>>();
    let n = read1::<usize>() - 1;
    s.sort();

    let head = n / 5;
    let tail = n % 5;
    println!("{}{}", s[head], s[tail]);
}