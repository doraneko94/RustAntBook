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
    let nt = readn::<usize>(" ");
    let n = nt[0];
    let _t = nt[1];

    let a = readn::<usize>(" ");
    let mut ans = 1;
    let mut profit = 0;
    let mut mi = a[0];
    for i in 1..n {
        mi = std::cmp::min(mi, a[i]);
        if a[i] - mi == profit {
            ans += 1;
        } else if a[i] - mi > profit {
            ans = 1;
            profit = a[i] - mi;
        }
    }

    println!("{}", ans);
}