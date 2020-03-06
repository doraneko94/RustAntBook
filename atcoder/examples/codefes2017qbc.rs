pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn dfs(i: usize, g: usize, edge: &Vec<Vec<usize>>, group: &mut Vec<usize>) -> bool {
    let g_inv = g ^ 1;
    group[i] = g_inv;

    for &j in edge[i].iter() {
        if group[j] == 2 {
            if !dfs(j, g_inv, edge, group) {
                return false;
            }
        } else if group[j] == g_inv { return false; }
    }

    true
}

fn main() {
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];

    let mut edge = vec![Vec::new(); n];
    for _ in 0..m {
        let ab = readn::<usize>(" ");
        edge[ab[0] - 1].push(ab[1] - 1);
        edge[ab[1] - 1].push(ab[0] - 1);
    }

    let mut group = vec![2; n];
    let mut flg = true;
    for i in 0..n {
        if group[i] == 2 {
            if !dfs(i, 0, &edge, &mut group) {
                flg = false;
                break;
            }
        }
    }

    let ans;
    if flg {
        let g1 = group.iter().sum::<usize>();
        let g2 = n - g1;
        ans = g1 * g2 - m;
    } else {
        ans = n * (n - 1) / 2 - m;
    }

    println!("{}", ans);
}