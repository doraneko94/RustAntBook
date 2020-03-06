use std::collections::BinaryHeap;

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
    let hw = readn::<usize>(" ");
    let h = hw[0];
    let w = hw[1];

    let _s = readn::<usize>(" ");
    let _g = readn::<usize>(" ");

    let field = (0..h).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let mut que = BinaryHeap::new();

    let mut uf = UnionFind::new(h*w);
    let mut ans = field.iter().map(|v| v.iter().sum::<usize>()).sum::<usize>();
    for y in 0..h {
        for x in 0..w-1 {
            que.push((field[y][x] * field[y][x+1], y*w+x, y*w+x+1));
        }
    }
    for y in 0..h-1 {
        for x in 0..w {
            que.push((field[y][x] * field[y+1][x], y*w+x, (y+1)*w+x));
        }
    }
    while let Some((p, a, b)) = que.pop() {
        if !uf.same(a, b) {
            uf.unite(a, b);
            ans += p;
        }
    }

    println!("{}", ans);
}