use std::collections::HashMap;

pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut dict = HashMap::new();
    let n = read1::<usize>();
    for _ in 0..n {
        let count = dict.entry(read1::<String>()).or_insert(0);
        *count += 1;
    }
    let m = read1::<usize>();
    for _ in 0..m {
        let count = dict.entry(read1::<String>()).or_insert(0);
        *count -= 1;
    }
    let ans = dict.iter().fold(0, |m, t| std::cmp::max(m, *t.1));
    println!("{:?}", ans);
}