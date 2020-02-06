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
    let ny = readn::<usize>(" ");
    let n = ny[0];
    let y = ny[1];

    for i in 0..(std::cmp::min(n, y / 10000) + 1) {
        for j in 0..(std::cmp::min(n - i, (y - 10000 * i) / 5000) + 1) {
            if 10000 * i + 5000 * j + 1000 * (n - i - j) == y {
                println!("{} {} {}", i, j, n - i - j);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}