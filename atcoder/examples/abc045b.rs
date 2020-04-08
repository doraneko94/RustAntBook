fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let sa = read1::<String>().chars().collect::<Vec<char>>();
    let sb = read1::<String>().chars().collect::<Vec<char>>();
    let sc = read1::<String>().chars().collect::<Vec<char>>();

    let mut ca = 1;
    let mut cb = 0;
    let mut cc = 0;
    let la = sa.len();
    let lb = sb.len();
    let lc = sc.len();
    let mut t = sa[0];

    loop {
        if t == 'a' {
            if ca == la {
                println!("A");
                return;
            } else {
                t = sa[ca];
                ca += 1;
            }
        } else if t == 'b' {
            if cb == lb {
                println!("B");
                return;
            } else {
                t = sb[cb];
                cb += 1;
            }
        } else {
            if cc == lc {
                println!("C");
                return;
            } else {
                t = sc[cc];
                cc += 1;
            }
        }
    }
}