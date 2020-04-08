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
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];
    let mut ans = k;
    for _ in 1..n {
        ans *= k - 1;
    }

    println!("{}", ans);
}