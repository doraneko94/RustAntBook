pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn dfs(i: usize, c: usize, ans: &mut Vec<usize>, edge: &Vec<Vec<(usize, usize)>>) {
    for e in edge[i].iter() {
        let j = e.0;
        let w = e.1;
        if ans[j] == 2 {
            if w % 2 == 0 {
                ans[j] = c;
                dfs(j, c, ans, edge);
            } else {
                ans[j] = c ^ 1;
                dfs(j , c ^ 1, ans, edge);
            }
        }
    }
}

fn main() {
    let n = read1::<usize>();
    let mut edge = vec![Vec::new(); n];
    let mut ans = vec![2; n];

    for _ in 0..n-1 {
        let uvw = readn::<usize>(" ");
        let u = uvw[0] - 1;
        let v = uvw[1] - 1;
        let w = uvw[2];

        edge[u].push((v, w));
        edge[v].push((u, w));
    }

    ans[0] = 0;
    dfs(0, 0, &mut ans, &edge);

    for c in ans.iter() {
        println!("{}", c);
    }
}