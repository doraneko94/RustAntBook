fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let mut ans = 0;
    
    for i in 1..n+1 {
        ans += i;
    }

    println!("{}", ans);
}