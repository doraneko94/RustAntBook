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
    let n = read1::<usize>();
    let mut xy = (0..n).map(|i| {
        let temp = readn::<usize>(" ");
        (temp[0], temp[1], i)
    }).collect::<Vec<(usize, usize, usize)>>();
    let mut yx = xy.iter().map(|&t| (t.1, t.0, t.2)).collect::<Vec<(usize, usize, usize)>>();

    xy.sort();
    yx.sort();

    let mut edge = Vec::with_capacity((n - 1) * 2);
    for i in 0..(n-1) {
        let xy0 = xy[i];
        let xy1 = xy[i+1];
        edge.push((xy0.2, xy1.2, xy1.0 - xy0.0));
        let yx0 = yx[i];
        let yx1 = yx[i+1];
        edge.push((yx0.2, yx1.2, yx1.0 - yx0.0));
    }
    edge.sort_by(|a, b| (a.2).cmp(&(b.2)));

    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for &e in edge.iter() {
        let a = e.0;
        let b = e.1;
        let c = e.2;
        if !uf.same(a, b) {
            uf.unite(a, b);
            ans += c;
        }
    }

    println!("{}", ans);
}