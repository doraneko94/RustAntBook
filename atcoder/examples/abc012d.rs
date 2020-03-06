fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: isize = 1_000_000_000;

fn main() {
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];

    let mut dp = vec![vec![vec![INF; n]; n]; n + 1];
    for i in 0..n {
        dp[0][i][i] = 0;
    }
    for _ in 0..m {
        let abt = readn::<usize>(" ");
        let a = abt[0] - 1;
        let b = abt[1] - 1;
        let t = abt[2] as isize;

        dp[0][a][b] = t;
        dp[0][b][a] = t;
    }

    for k in 1..n+1 {
        for i in 0..n {
            for j in 0..n {
                dp[k][i][j] = std::cmp::min(dp[k-1][i][j], dp[k-1][i][k-1] + dp[k-1][k-1][j]);
            }
        }
    }

    let mut maxl = INF;
    for i in 0..n {
        let cand = dp[n][i].iter().fold(-1, |m, &e| std::cmp::max(m, e));
        maxl = std::cmp::min(maxl, cand);
    }
    println!("{}", maxl);
}