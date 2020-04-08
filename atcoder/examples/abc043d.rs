fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let mut ans = -1;
    if s.len() == 2 {
        if s[0] == s[1] {
            println!("1 2");
        } else {
            println!("-1 -1");
        }
        return;
    }
    for l in 0..s.len()-2 {
        if s[l] == s[l+1] || s[l] == s[l+2] || s[l+1] == s[l+2] {
            ans = l as isize;
            break;
        }
    }
    if ans >= 0 {
        println!("{} {}", ans+1, ans+3);
    } else {
        println!("-1 -1");
    }
}