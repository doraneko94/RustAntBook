fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let mut c = s[0];
    let mut ans = 0usize;
    for i in 1..s.len() {
        if c != s[i] {
            c = s[i];
            ans += 1;
        }
    }

    println!("{}", ans);
}