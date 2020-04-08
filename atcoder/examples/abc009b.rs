use std::collections::HashSet;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let mut set = HashSet::new();

    for _ in 0..n {
        let a = read1::<usize>();
        set.insert(a);
    }

    let mut v = set.iter().map(|&i| i).collect::<Vec<usize>>();
    v.sort_by(|a, b| b.cmp(&a));

    println!("{}", v[1]);
}