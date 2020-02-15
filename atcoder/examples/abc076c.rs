pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let t = read1::<String>().chars().collect::<Vec<char>>();
    let s_len = s.len();
    let t_len = t.len();
    if s_len < t_len {
        println!("UNRESTORABLE");
        return;
    }
    let mut pos = s_len;
    for i in 0..(s_len - t_len + 1) {
        let mut flg = true;
        for j in 0..t_len {
            if s[i+j] != t[j] && s[i+j] != '?' {
                flg = false;
                break;
            }
        }
        if flg {
            pos = i;
        }
    }
    if pos != s_len {
        let ans = s.iter().enumerate().map(|(i, &c)| {
            if i >= pos && i < pos + t_len {
                t[i - pos]
            } else {
                if c == '?' {
                    'a'
                } else {
                    s[i]
                }
            }
        }).collect::<Vec<char>>();
        println!("{}", ans.into_iter().collect::<String>());
    } else {
        println!("UNRESTORABLE");
    }
}