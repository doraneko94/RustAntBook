pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let s = n.to_string().chars().collect::<Vec<char>>();
    let mut ans = "NO";

    if n % 3 == 0 {
        ans = "YES";
    }
    for &c in s.iter() {
        if c == '3' {
            ans = "YES";
            break;
        }
    }

    println!("{}", ans);
}