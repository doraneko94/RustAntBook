use std::collections::HashSet;

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
    let n = read1::<usize>();
    let a = readn::<usize>(" ");
    let mut set = HashSet::new();
    let mut v = Vec::new();

    for i in 0..n {
        if set.contains(&a[i]) {
            v.push(a[i]);
            set.remove(&a[i]);
        } else {
            set.insert(a[i]);
        }
    }

    v.sort();
    let l = v.len();
    if l < 2 {
        println!("0");
    } else {
        println!("{}", v[l-1] * v[l-2]);
    }
}