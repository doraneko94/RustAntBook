fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn f(a: usize, b: usize) -> usize {
    let x;
    let y;
    if a < b {
        x = a;
        y = b;
    } else {
        y = a;
        x = b;
    } 
    if x == 0 {
        return 0;
    }
    if y % x == 0 {
        return 2 * y - x;
    }
    2 * x * (y / x) + f(y % x, x)
}

fn main() {
    let nx = readn::<usize>(" ");
    let n = nx[0];
    let x = nx[1];

    println!("{}", n + f(n - x, x));
}