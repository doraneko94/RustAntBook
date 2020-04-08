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
    let k = nk[1];
    let mut r = readn::<usize>(" ");
    r.sort_by(|a, b| b.cmp(&a));

    let mut ans = 0f64;
    for i in (0..k).rev() {
        ans = (ans + (r[i] as f64)) / 2.0;
    }

    println!("{}", ans);
}