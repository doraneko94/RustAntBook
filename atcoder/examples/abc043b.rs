fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();

    let mut ans = "".to_string();
    for &c in s.iter() {
        if c == 'B'  {
            ans.pop();
        } else {
            ans.push(c);
        }
    }

    println!("{}", ans);
}