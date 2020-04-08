use std::cmp::min;

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
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];

    let x = readn::<isize>(" ");
    let mut ans = 1_000_000_000;
    for l in 0..n-k+1 {
        ans = min(min(x[l].abs() + (x[l] - x[l+k-1]).abs(), x[l+k-1].abs() + (x[l] - x[l+k-1]).abs()), ans);
    }
    println!("{}", ans);
}