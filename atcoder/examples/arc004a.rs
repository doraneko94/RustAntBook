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
    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    
    for _ in 0..n {
        let xy = readn::<f64>(" ");
        x.push(xy[0]);
        y.push(xy[1]);
    }

    let mut ans = 0.0;
    for i in 0..n {
        for j in 0..n {
            let dist = (x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j]);
            if ans < dist {
                ans = dist;
            }
        }
    }
    println!("{:.06}", ans.sqrt());
}