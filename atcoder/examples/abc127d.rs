use std::collections::HashMap;

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
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];

    let mut a = readn::<usize>(" ");
    let mut dict = HashMap::new();
    for &i in a.iter() {
        let count = dict.entry(i).or_insert(0);
        *count += 1;
    }

    for _ in 0..m {
        let bc = readn::<usize>(" ");
        let b = bc[0];
        let c = bc[1];
        let count = dict.entry(c).or_insert(0);
        *count += b;
        a.push(c);
    }

    a.sort_by(|x, y| y.cmp(&x));
    let mut count = 0;
    let mut ans = 0;
    let mut pre = 0;
    for i in 0..a.len() {
        if a[i] == pre {
            continue;
        }
        let num = dict[&a[i]];
        if num + count > n {
            ans += (n - count) * a[i];
            break;
        } else {
            ans += num * a[i];
            count += num;
            pre = a[i];
        }
    }
    println!("{}", ans);
}