use std::collections::HashSet;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let abc = readn::<usize>(" ");
    let mut set = HashSet::new();
    for i in 0..3 {
        set.insert(abc[i]);
    }
    println!("{}", set.len());
}