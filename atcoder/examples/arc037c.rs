use std::cmp::Ordering;

pub fn upper_bound<T: Ord>(sorted: &[T], x: &T) -> usize {
    let mut low = 0;
    let mut high = sorted.len();

    while low != high {
        let mid = (low + high) / 2;
        match sorted[mid].cmp(x) {
            Ordering::Less | Ordering::Equal => low = mid + 1,
            Ordering::Greater => high = mid,
        };
    }

    low
}

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1_000_000_000_000_000_000;

fn main() {
    let nk = readn::<usize>(" ");
    let _n = nk[0];
    let k = nk[1];
    let mut a = readn::<usize>(" ");
    let mut b = readn::<usize>(" ");

    a.sort();
    b.sort();

    let mut low = 0;
    let mut high = INF;

    while low != high {
        let mid = (low + high) / 2;
        let s = b.iter().map(|&bi| {
            let ai = mid / bi;
            upper_bound(&a, &ai)
        }).sum::<usize>();
        if s < k {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    println!("{}", low);
}