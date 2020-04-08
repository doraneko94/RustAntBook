use std::cmp::{min, max};

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

    let mut l = 1;
    let mut r = n;

    for _ in 0..m {
        let lr = readn::<usize>(" ");
        l = max(l, lr[0]);
        r = min(r, lr[1]);
    }

    if l > r {
        println!("0");
    } else {
        println!("{}", r - l + 1);
    }
}