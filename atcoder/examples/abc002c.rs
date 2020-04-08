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
    let xy = readn::<isize>(" ");
    let a = xy[2] - xy[0];
    let b = xy[3] - xy[1];
    let c = xy[4] - xy[0];
    let d = xy[5] - xy[1];

    let ans = (a * d - b * c).abs() as f64 / 2.0;
    println!("{:.03}", ans);
}