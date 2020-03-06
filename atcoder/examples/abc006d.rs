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
    let mut dp = Vec::new();
    for _ in 0..n {
        let c = read1::<usize>();
        let index = match dp.binary_search(&c) {
            Ok(e) => e,
            Err(e) => e,
        };
        if index < dp.len() {
            dp[index] = c;
        } else {
            dp.push(c);
        }
    }

    println!("{}", n - dp.len());
}