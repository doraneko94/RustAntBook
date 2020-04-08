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
    let n = read1::<usize>();
    let xy = (0..n).map(|_| readn::<f64>(" ")).collect::<Vec<Vec<f64>>>();

    let mut ans = 0.0;
    for i in 0..n {
        for j in i+1..n {
            ans += 2.0 * ((xy[i][0] - xy[j][0]) * (xy[i][0] - xy[j][0]) + (xy[i][1] - xy[j][1]) * (xy[i][1] - xy[j][1])).sqrt();
        }
    }

    println!("{:?}", ans / n as f64);
}