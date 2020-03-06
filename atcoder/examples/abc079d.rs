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

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1_000_000_000;

fn main() {
    let hw = readn::<usize>(" ");
    let h = hw[0];
    let _w = hw[1];
    let c = (0..10).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let mut count = vec![0; 10];
    for _ in 0..h {
        let av = readn::<isize>(" ");
        for &a in av.iter() {
            if a >= 0 {
                count[a as usize] += 1;
            }
        }
    }

    let mut dist = vec![INF; 10];
    let mut que = BinaryHeap::new();
    dist[1] = 0;
    que.push(Rev((0, 1)));
    while let Some(Rev((d, i))) = que.pop() {
        if dist[i] != d {
            continue;
        }
        for j in 0..10 {
            if j == i {
                continue;
            }
            if dist[j] > dist[i] + c[j][i] {
                dist[j] = dist[i] + c[j][i];
                que.push(Rev((dist[j], j)));
            }
        }
    }

    let mut ans = 0;
    for i in 0..10 {
        ans += dist[i] * count[i];
    }

    println!("{}", ans);
}