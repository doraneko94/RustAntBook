use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1_000_000_000_000;

fn main() {
    let nmt = readn::<usize>(" ");
    let n = nmt[0];
    let m = nmt[1];
    let t = nmt[2];

    let mut d1 = vec![INF; n];
    let mut d2 = vec![INF; n];
    let mut e1 = vec![Vec::new(); n];
    let mut e2 = vec![Vec::new(); n];
    let a = readn::<usize>(" ");

    for _ in 0..m {
        let abc = readn::<usize>(" ");
        e1[abc[0] - 1].push((abc[1] - 1, abc[2]));
        e2[abc[1] - 1].push((abc[0] - 1, abc[2]));
    }

    let mut que = BinaryHeap::new();
    que.push(Rev((0, 0)));
    d1[0] = 0;
    while que.len() > 0 {
        let rt = que.pop().unwrap();
        let t = rt.0;
        let d = t.0;
        let i = t.1;
        if d != d1[i] { continue; }
        for &e in e1[i].iter() {
            let j = e.0;
            let dd = e.1;
            if d1[j] > d + dd {
                que.push(Rev((d + dd, j)));
                d1[j] = d + dd;
            }
        }
    }
    let mut que = BinaryHeap::new();
    que.push(Rev((0, 0)));
    d2[0] = 0;
    while que.len() > 0 {
        let rt = que.pop().unwrap();
        let t = rt.0;
        let d = t.0;
        let i = t.1;
        if d != d2[i] { continue; }
        for &e in e2[i].iter() {
            let j = e.0;
            let dd = e.1;
            if d2[j] > d + dd {
                que.push(Rev((d + dd, j)));
                d2[j] = d + dd;
            }
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let time = d1[i] + d2[i];
        if time > t { continue; }
        else {
            ans = std::cmp::max(ans, (t - time) * a[i]);
        }
    }
    println!("{}", ans);
}