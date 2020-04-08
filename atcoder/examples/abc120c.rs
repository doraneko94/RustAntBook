use std::collections::HashMap;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let mut dict = HashMap::new();

    for &c in s.iter() {
        let count = dict.entry(c).or_insert(0);
        *count += 1;
    }
    match dict.get(&'0') {
        Some(n) => {
            match dict.get(&'1') {
                Some(m) => {
                    println!("{}", std::cmp::min(n, m) * 2);
                }
                _ => {
                    println!("0");
                }
            };
        }
        _ => {
            println!("0");
        }
    };
}