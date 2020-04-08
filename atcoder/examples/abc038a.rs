fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    if s[s.len() - 1] == 'T' {
        println!("YES");
    } else {
        println!("NO");
    }
}