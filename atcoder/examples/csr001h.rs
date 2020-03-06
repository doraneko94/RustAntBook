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
    let a = readn::<usize>(" ");

    let mut dp = Vec::new();
    dp.push(a[0]);
    for i in 1..n {
        let index = match dp.binary_search(&a[i]) {
            Ok(e) => e,
            Err(e) => e,
        };
        if index < dp.len() {
            dp[index] = a[i];
        } else {
            dp.push(a[i]);
        }
    }

    println!("{}", dp.len());
}