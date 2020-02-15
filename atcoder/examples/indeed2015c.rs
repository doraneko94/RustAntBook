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

    let mut dp = vec![vec![vec![0; 101]; 101]; 101];

    for _ in 0..n {
        let abcw = readn::<usize>(" ");
        dp[abcw[0]][abcw[1]][abcw[2]] = std::cmp::max(dp[abcw[0]][abcw[1]][abcw[2]], abcw[3]);
    }

    for a in 0..101 {
        for b in 0..101 {
            for c in 0..101 {
                let mut cand = dp[a][b][c];
                if a > 0 {
                    cand = std::cmp::max(cand, dp[a-1][b][c]);
                }
                if b > 0 {
                    cand = std::cmp::max(cand, dp[a][b-1][c]);
                }
                if c > 0 {
                    cand = std::cmp::max(cand, dp[a][b][c-1]);
                }
                dp[a][b][c] = cand;
            }
        }
    }

    for _ in 0..m {
        let xyz = readn::<usize>(" ");
        println!("{}", dp[xyz[0]][xyz[1]][xyz[2]]);
    }
}