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

const INF: usize = 10000000000000000;

fn main() {
    let nmx = readn::<usize>(" ");
    let n = nmx[0];
    let m = nmx[1];
    let x = nmx[2];

    let h = (0..n).map(|_| read1::<usize>()).collect::<Vec<usize>>();
    let mut edge = vec![Vec::new(); n];
    for _ in 0..m {
        let abt = readn::<usize>(" ");
        edge[abt[0]-1].push((abt[1]-1, abt[2]));
        edge[abt[1]-1].push((abt[0]-1, abt[2]));
    }

    let mut dp = vec![(INF, 0); n];
    let mut visited = vec![false; n];
    let mut heap = BinaryHeap::new();
    dp[0] = (0, x);
    heap.push(Rev((0, 0)));

    while heap.len() > 0 {
        let ds = heap.pop().unwrap().0;
        let d = ds.0;
        let s = ds.1;
        if dp[s].0 != d { continue; }
        if visited[s] { continue; }
        visited[s] = true;
        for i in 0..edge[s].len() {
            let e = edge[s][i];
            let g = e.0;
            let t = e.1;
            if visited[g] { continue; }
            if h[s] < t { continue; }

            let mut time = t;
            let mut h_now = dp[s].1;
            if h_now < t {
                time += t - h_now;
                h_now = t;
            } else if h_now - t > h[g] {
                time += h_now - (t + h[g]);
                h_now = t + h[g];
            }
            if dp[g].0 > dp[s].0 + time {
                dp[g].0 = dp[s].0 + time;
                dp[g].1 = h_now - t;
                heap.push(Rev((dp[g].0, g)));
            }
        }
    }
    if dp[n-1].0 < INF {
        println!("{}", dp[n-1].0 + h[n-1] - dp[n-1].1);
    } else {
        println!("-1");
    }
}