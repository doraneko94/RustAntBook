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

const INF: usize = 1_000_000_000_000_000;

fn main() {
    let nmst = readn::<usize>(" ");
    let n = nmst[0];
    let m = nmst[1];
    let s = nmst[2] - 1;
    let t = nmst[3] - 1;

    let mut yen = vec![Vec::new(); n];
    let mut snuke = vec![Vec::new(); n];

    for _ in 0..m {
        let uvab = readn::<usize>(" ");
        let u = uvab[0] - 1;
        let v = uvab[1] - 1;
        let a = uvab[2];
        let b = uvab[3];

        yen[u].push((v, a));
        yen[v].push((u, a));
        snuke[u].push((v, b));
        snuke[v].push((u, b));
    }

    let mut ds = vec![INF; n];
    let mut dt = vec![INF; n];
    ds[s] = 0;
    dt[t] = 0;

    let mut que = BinaryHeap::new();
    que.push(Rev((0, s)));
    while let Some(Rev((d, i))) = que.pop() {
        if ds[i] != d { continue; }
        for &tup in yen[i].iter() {
            let j = tup.0;
            let dd = tup.1;
            if ds[j] > d + dd {
                ds[j] = d + dd;
                que.push(Rev((d + dd, j)));
            }
        }
    }

    let mut que = BinaryHeap::new();
    que.push(Rev((0, t)));
    while let Some(Rev((d, i))) = que.pop() {
        if dt[i] != d { continue; }
        for &tup in snuke[i].iter() {
            let j = tup.0;
            let dd = tup.1;
            if dt[j] > d + dd {
                dt[j] = d + dd;
                que.push(Rev((d + dd, j)));
            }
        }
    }

    let mut que = BinaryHeap::new();
    for i in 0..n {
        let cost = ds[i] + dt[i];
        if cost > INF {
            que.push((0, i));
        } else {
            que.push((INF - cost, i));
        }
    }

    let mut di = que.pop().unwrap();
    let mut ans = String::new();
    for i in 0..n {
        while di.1 < i {
            di = que.pop().unwrap();
        }
        ans += &((di.0).to_string());
        ans += "\n";
    }
    println!("{}", ans);
}