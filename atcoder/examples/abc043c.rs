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
    let n = read1::<isize>();
    let a = readn::<isize>(" ");

    let s2 = a.iter().map(|x| x*x).sum::<isize>();
    let s1 = a.iter().sum::<isize>();
    let m1 = s1 / n;
    let m2 = (s1 + n - 1) / n;

    let ans = std::cmp::min(n * m1 * m1 + s2 - 2 * s1 * m1, n * m2 * m2 + s2 - 2 * s1 * m2);
    println!("{}", ans);
}