fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn judge(t: usize, n: usize, x: &Vec<usize>) -> bool {
    let m = x.len();
    let mut l = x[0] - 1;
    for i in 0..m-1 {
        let mut low = 0;
        let mut high = x[i+1] - x[i];
        if l > t {
            return false;
        }
        while low != high {
            let mid = (low + high) / 2;
            if std::cmp::min(l + 2 * mid, 2 * l + mid) <= t {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        l = x[i+1] - x[i] - low;
    }
    let r = n - x[m-1];
    if std::cmp::min(l + 2 * r, 2 * l + r) > t {
        false
    } else {
        true
    }
}

fn main() {
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];

    let x = (0..m).map(|_| read1::<usize>()).collect::<Vec<usize>>();
    let mut low = 0;
    let mut high = n * 3 / 2 + 1;
    while low != high {
        let mid = (low + high) / 2;
        if judge(mid, n, &x) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    println!("{}", low);
}