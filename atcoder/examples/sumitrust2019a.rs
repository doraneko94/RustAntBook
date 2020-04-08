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
    let m1d1 = readn::<usize>(" ");
    let m1 = m1d1[0];
    let m2d2 = readn::<usize>(" ");
    let m2 = m2d2[0];

    if m1 == m2 {
        println!("0");
    } else {
        println!("1");
    }
}