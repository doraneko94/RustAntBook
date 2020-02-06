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
    let mut visited = vec![0; n];
    let mut edge = vec![Vec::new(); n];

    for _ in 0..m {
        let ab = readn::<usize>(" ");
        edge[ab[0] - 1].push(ab[1] - 1);
        edge[ab[1] - 1].push(ab[0] - 1);
    }

    println!("{}", dfs(0, 0, n, &mut visited, &edge));
}

fn dfs(node: usize, num: usize, n: usize, visited: &mut Vec<usize>, edge: &Vec<Vec<usize>>) -> usize {
    if num + 1 == n {
        return 1;
    }

    let mut v = (*visited).clone();
    let mut path = 0;
    v[node] = 1;

    for &b in (*edge[node]).iter() {
        if visited[b] == 0 {
            path += dfs(b, num + 1, n, &mut v, edge);
        }
    }

    path
}