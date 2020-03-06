fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn judge(x: f64, wp: &Vec<Vec<f64>>, k: usize) -> bool {
    let mut tmp = wp.iter().map(|v| v[0] * (v[1] - x)).collect::<Vec<f64>>();
    tmp.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut ans = 0.0;
    for i in 0..k {
        ans += tmp[i];
    }
    if ans >= 0.0 {
        true
    } else {
        false
    }
}

fn main() {
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];

    let mut wp = Vec::with_capacity(n);
    for _ in 0..n {
        let temp = readn::<f64>(" ");
        wp.push(vec![temp[0], temp[1] / 100.0]);
    }

    let mut low = 0.0;
    let mut high = 1.0;
    while (high - low) >= 0.0000000000001 {
        let mid = (high + low) / 2.0;
        if judge(mid, &wp, k) {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("{:.10}", low * 100.0);
}