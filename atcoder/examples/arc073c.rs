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
    let nt = readn::<usize>(" ");
    let n = nt[0];
    let t = nt[1];

    let vt = readn::<usize>(" ");
    let mut ans = 0;

    for i in 1..n {
        if vt[i] - vt[i-1] > t {
            ans += t;
        } else {
            ans += vt[i] - vt[i-1];
        }
    }

    println!("{}", ans + t);
}