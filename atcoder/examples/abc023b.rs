fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let s = read1::<String>().chars().collect::<Vec<char>>();

    if n % 2 == 0 {
        println!("-1");
    } else {
        let m = n / 2;
        if s[m] != 'b' {
            println!("-1");
        } else {
            let mut flg = true;
            for i in 0..m+1 {
                if i % 3 == 1 {
                    if s[m - i] != 'a' || s[m + i] != 'c' {
                        flg = false;
                        break;
                    }
                } else if i % 3 == 2 {
                    if s[m - i] != 'c' || s[m + i] != 'a' {
                        flg = false;
                        break;
                    }
                } else {
                    if s[m - i] != 'b' || s[m + i] != 'b' {
                        flg = false;
                        break;
                    }
                }
            }
            if flg {
                println!("{}", m);
            } else {
                println!("-1");
            }
        }
    }
}