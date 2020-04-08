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
    let n = read1::<usize>();
    let mut mt = 0;
    let mut ma = 0;
    for _ in 0..n {
        let ta = readn::<usize>(" ");
        let t = ta[0];
        let a = ta[1];
        let m = std::cmp::max((mt + t - 1)/ t, (ma + a - 1) / a);
        if mt == 0 && ma == 0 {
            mt = t;
            ma = a;
        } else {
            mt = t * m;
            ma = a * m;
        }
    }

    println!("{}", mt + ma);
}