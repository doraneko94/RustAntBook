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
    let nq = readn::<usize>(" ");
    let n = nq[0];
    let q = nq[1];
    let mut uf = UnionFind::new(n);
    let mut ans = String::new();

    for _ in 0..q {
        let pab = readn::<usize>(" ");
        let p = pab[0];
        let a = pab[1];
        let b = pab[2];

        if p == 0 {
            uf.unite(a, b);
        } else {
            if uf.same(a, b) {
                ans += "Yes\n";
            } else {
                ans += "No\n";
            }
        }
    }
    println!("{}", ans);
}