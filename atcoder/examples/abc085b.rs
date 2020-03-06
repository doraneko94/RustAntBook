pub fn read1<T: std::str::FromStr>() -> T {
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
    let mut u = Vec::new();

    for _ in 0..n {
        let d = read1::<usize>();
        match u.binary_search(&d) {
            Ok(_) => continue,
            Err(i) => u.insert(i, d),
        };
    }

    println!("{}", u.len());
}