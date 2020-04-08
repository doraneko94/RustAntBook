pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let mut r = (0..n).map(|_| read1::<usize>()).collect::<Vec<usize>>();

    r.sort_by(|a, b| b.cmp(&a));

    let mut sum = 0;
    for i in 0..n {
        if i % 2 == 0 {
            sum += r[i] * r[i];
        } else {
            sum -= r[i] * r[i];
        }
    }

    println!("{:.08}", sum as f64 * std::f64::consts::PI);
}