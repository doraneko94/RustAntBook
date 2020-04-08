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
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];

    if m < 2 * n || m > 4 * n {
        println!("-1 -1 -1");
    } else {
        let rest = m - 2 * n;
        let c = rest / 2;
        let b = rest % 2;
        let a = n - b - c;
        println!("{} {} {}", a, b, c);
    }
}