use std::collections::HashMap;

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
    let nkl = readn::<usize>(" ");
    let n = nkl[0];
    let k = nkl[1];
    let l = nkl[2];

    let mut uf1 = UnionFind::new(n);
    let mut uf2 = UnionFind::new(n);
    let mut dict = HashMap::new();
    
    for _ in 0..k {
        let pq = readn::<usize>(" ");
        uf1.unite(pq[0]-1, pq[1]-1);
    }
    let root1 = (0..n).map(|i| uf1.root(i)).collect::<Vec<usize>>();

    for _ in 0..l {
        let pq = readn::<usize>(" ");
        uf2.unite(pq[0]-1, pq[1]-1);
    }
    let root2 = (0..n).map(|i| uf2.root(i)).collect::<Vec<usize>>();

    for i in 0..n {
        let t = (root1[i], root2[i]);
        let count = dict.entry(t).or_insert(0);
        *count += 1;
    }

    let mut ans = String::new();
    for i in 0..n {
        let add = dict[&(root1[i], root2[i])].to_string();
        ans += &add;
        ans += " ";
    }
    println!("{}", ans);
}