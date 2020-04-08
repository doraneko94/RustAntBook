fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let n = s.len();
    let m = n / 2;
    let l = s.iter().map(|&c| {
        if c == 'g' { 0 }
        else { 1 }
    }).sum::<usize>();
    println!("{}", m - l);
}