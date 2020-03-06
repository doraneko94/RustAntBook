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

pub struct UnionFind {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let par = (0..n).collect::<Vec<usize>>();
        let rank = vec![0; n];
        UnionFind { par: par, rank: rank }
    }

    pub fn root(&mut self, x: usize) -> usize {
        let cand = self.par[x];
        if cand == x {
            return x;
        } else {
            let ret = self.root(cand);
            self.par[x] = ret;
            return ret;
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x == root_y { return; }

        if self.rank[root_x] < self.rank[root_y] {
            self.par[root_x] = self.par[root_y];
        } else {
            self.par[root_y] = self.par[root_x];
            if self.rank[root_y] == self.rank[root_x] {
                self.rank[root_x] += 1;
            }
        }
    }
}

fn main() {
    let mut ans = String::new();
    let mut flg = false;
    loop {
        let nm = readn::<usize>(" ");
        let n = nm[0];
        let m = nm[1];
        if n == 0 && m == 0 {
            println!("{}", ans);
            return;
        }

        let mut que = BinaryHeap::new();
        let mut uf = UnionFind::new(n);
        for _ in 0..m {
            let stc = readn::<usize>(" ");
            let s = stc[0] - 1;
            let t = stc[1] - 1;
            let c = stc[2];
            que.push(Rev((c, s, t)));
        }

        let mut count = 0;
        let med = n / 2;
        while let Some(Rev((c, s, t))) = que.pop() {
            if !uf.same(s, t) {
                count += 1;
                if count == med {
                    if flg {
                        ans += "\n";
                    } else {
                        flg = true;
                    }
                    ans += &(c.to_string());
                    break;
                }
                uf.unite(s, t);
            }
        }
    }
}