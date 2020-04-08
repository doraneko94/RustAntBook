use std::collections::HashMap;

fn read1<T: std::str::FromStr>() -> T {
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
    let nmk = readn::<usize>(" ");
    let n = nmk[0];
    let m = nmk[1];
    let k = nmk[2];

    let mut uf = UnionFind::new(n);
    let mut loss = vec![1; n];
    for _ in 0..m {
        let ab = readn::<usize>(" ");
        let a = ab[0] - 1;
        let b = ab[1] - 1;
        uf.unite(a, b);
        loss[a] += 1;
        loss[b] += 1;
    }

    let mut dict = HashMap::new();
    for i in 0..n {
        let count = dict.entry(uf.root(i)).or_insert(0usize);
        *count += 1;
    }

    for _ in 0..k {
        let cd = readn::<usize>(" ");
        let c = cd[0] - 1;
        let d = cd[1] - 1;
        if uf.same(c, d) {
            loss[c] += 1;
            loss[d] += 1;
        }
    }

    let mut ans = "".to_string();
    for i in 0..n {
        let num = dict.get(&uf.root(i)).unwrap() - loss[i];
        ans = ans + &(num.to_string()) + " ";
    }
    println!("{}", ans);
}