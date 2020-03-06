fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn judge(hs: &Vec<Vec<usize>>, lim: usize) -> bool {
    let n = hs.len();
    let mut t = Vec::with_capacity(n);
    for v in hs.iter() {
        if v[0] > lim {
            return false;
        }
        t.push((lim - v[0]) / v[1]);
    }
    t.sort();
    for i in 0..n {
        if t[i] < i {
            return false;
        }
    }
    true
}

fn main() {
    let n = read1::<usize>();
    let hs = (0..n).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let mut cand = hs.iter().map(|v| v[0] + (n-1) * v[1]).collect::<Vec<usize>>();
    
    cand.sort();
    let mut low = cand[0];
    let mut high = cand[n-1];

    while low != high {
        let mid = (low + high) / 2;
        if judge(&hs, mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    println!("{}", low);
}