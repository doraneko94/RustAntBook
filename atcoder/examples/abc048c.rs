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
    let nx = readn::<usize>(" ");
    let n = nx[0];
    let x = nx[1];
    let mut a = readn::<usize>(" ");

    let mut ans = if a[0] > x {
        a[0] - x
    } else {
        0usize
    };
    a[0] = std::cmp::min(a[0], x);
    for i in 1..n {
        if a[i] + a[i-1] > x {
            let d = a[i] + a[i-1] - x;
            ans += d;
            a[i] -= d;
        }
    }

    println!("{}", ans);
}