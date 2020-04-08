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
    let n = read1::<usize>();
    let a = readn::<usize>(" ");

    let mut sum = 0;
    let mut m = 0;
    for i in 0..n {
        if a[i] > 0 {
            sum += a[i];
            m += 1;
        }
    }

    println!("{}", (sum + m - 1) / m);
}