fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let t1t2 = readn::<usize>(" ");
    let t1 = t1t2[0];
    let t2 = t1t2[1];
    let a1a2 = readn::<usize>(" ");
    let a1 = a1a2[0];
    let a2 = a1a2[1];
    let b1b2 = readn::<usize>(" ");
    let b1 = b1b2[0];
    let b2 = b1b2[1];

    let t1a1 = t1 * a1;
    let t2a2 = t2 * a2;
    let t1b1 = t1 * b1;
    let t2b2 = t2 * b2;

    if t1a1 == t1b1 && t2a2 == t2b2 {
        println!("infinity");
    } else if t1a1 >= t1b1 {
        let ab1 = t1a1 - t1b1;
        if t2a2 >= t2b2 {
            println!("0");
        } else {
            let ab2 = t2b2 - t2a2;
            if ab1 == ab2 {
                println!("infinity");
            } else {
                if ab1 > ab2 {
                    println!("0");
                } else {
                    let r = ab2 - ab1;
                    let mut ans = 1;
                    let t = (ab1 + r - 1) / r;
                    if ab1 % r == 0 {
                        ans += (t - 1) * 2 + 1;
                    } else {
                        ans += (t - 1) * 2;
                    }
                    println!("{}", ans);
                }
            }
        }
    } else {
        let ab1 = t1b1 - t1a1;
        if t2a2 <= t2b2 {
            println!("0");
        } else {
            let ab2 = t2a2 - t2b2;
            if ab1 == ab2 {
                println!("infinity");
            } else {
                if ab1 > ab2 {
                    println!("0");
                } else {
                    let r = ab2 - ab1;
                    let mut ans = 1;
                    let t = (ab1 + r - 1) / r;
                    if ab1 % r == 0 {
                        ans += (t - 1) * 2 + 1;
                    } else {
                        ans += (t - 1) * 2;
                    }
                    println!("{}", ans);
                }
            }
        }
    }
}