use std::cmp::Ordering;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn lower_bound<T: Ord>(sorted: &[T], x: &T) -> usize {
    let mut low = 0;
    let mut high = sorted.len();

    while low != high {
        let mid = (low + high) / 2;
        match sorted[mid].cmp(x) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater | Ordering::Equal => high = mid,
        };
    }

    low
}

fn main() {
    let n = read1::<usize>();
    let mut a = readn::<usize>(" ");
    let mut b = readn::<usize>(" ");
    let mut c = readn::<usize>(" ");

    a.sort();
    b.sort();
    c.sort();

    let mut an = vec![0; n];
    let mut bn = vec![0; n];

    for i in 0..n {
        let bni = lower_bound(&b, &(a[i] + 1));
        an[i] = bni;
    }

    for i in 0..n {
        let cni = lower_bound(&c, &(b[i] + 1));
        bn[i] = cni;
    }

    let mut bs = vec![0; n + 1];
    for i in 1..n+1 {
        bs[n-i] = bs[n-i+1] + (n - bn[n-i]);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += bs[an[i]];
    }
    println!("{}", ans);
}