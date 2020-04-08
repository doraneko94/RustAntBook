fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let s = read1::<String>().chars().collect::<Vec<char>>();

    let mut ans = 0;
    
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let v = vec![i, j, k];
                let mut now = 0;
                for l in 0..n {
                    if s[l] as usize - 48 == v[now] {
                        if now == 2 {
                            ans += 1;
                            break;
                        } else {
                            now += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("{}", ans);
}