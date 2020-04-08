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
    let abk = readn::<usize>(" ");
    let mut a = abk[0];
    let mut b = abk[1];
    let k = abk[2];

    for i in 0..k {
        if i % 2 == 0 {
            a /= 2;
            b += a;
        } else {
            b /= 2;
            a += b;
        }
    }

    println!("{} {}", a, b);
}