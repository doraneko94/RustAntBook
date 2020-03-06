use std::f64::consts::PI;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn f(a: f64, b: f64, c: f64, t: f64) -> f64 {
    a * t + b * (c * t * PI).sin()
}

fn main() {
    let abc = readn::<f64>(" ");
    let a = abc[0];
    let b = abc[1];
    let c = abc[2];

    let mut low = 0.0;
    let mut high = b + 100.0;

    while (f(a, b, c, low) - 100.0).abs() > 0.00000001 {
        let mid = (low + high) / 2.0;
        if f(a, b, c, mid) < 100.0 {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("{}", low);
}