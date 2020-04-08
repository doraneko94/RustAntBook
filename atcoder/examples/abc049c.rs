fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let mut p = s.len();
    while p >= 5 {
        let mut flg = false;
        if p >= 7 {
            for (i, &c) in vec!['r', 'e', 'm', 'a', 'e', 'r', 'd'].iter().enumerate() {
                if s[p-i-1] != c { break; }
                if i == 6 {
                    flg = true;
                }
            }
            if flg {
                p -= 7;
                continue;
            }
        }
        if p >= 6 {
            for (i, &c) in vec!['r', 'e', 's', 'a', 'r', 'e'].iter().enumerate() {
                if s[p-i-1] != c { break; }
                if i == 5 {
                    flg = true;
                }
            }
            if flg {
                p -= 6;
                continue;
            }
        }
        
        for (i, &c) in vec!['m', 'a', 'e', 'r', 'd'].iter().enumerate() {
            if s[p-i-1] != c { break; }
            if i == 4 {
                flg = true;
            }
        }
        if flg {
            p -= 5;
            continue;
        }
        for (i, &c) in vec!['e', 's', 'a', 'r', 'e'].iter().enumerate() {
            if s[p-i-1] != c { break; }
            if i == 4 {
                flg = true;
            }
        }
        if flg {
            p -= 5;
        } else {
            break;
        }
    }
    if p == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}