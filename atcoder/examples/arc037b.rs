pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];
    let mut c = vec![0_usize; n];
    let mut e = vec![Vec::new(); n];

    for _ in 0..m {
        let uv = readn::<usize>(" ");
        e[uv[0] - 1].push(uv[1] - 1);
        e[uv[1] - 1].push(uv[0] - 1);
    }

    let mut ans = 0;

    for i in 0..n {
        if c[i] == 0 {
            if dfs(i, n + 1, &mut c, &e) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn dfs(u: usize, u0: usize, c: &mut [usize], e: &[Vec<usize>]) -> bool {
    if c[u] == 1 { return false };
    c[u] = 1;

    for &v in e[u].iter() {
        if v != u0 {
            if !dfs(v, u, c, e) {
                return false;
            }
        }
    }

    true
}