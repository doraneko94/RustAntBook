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
    let n = read1::<usize>();
    let mut dp = vec![vec![vec![vec![0; n]; n]; n]; n];
    for i in 0..n {
        let d = readn::<usize>(" ");
        for j in 0..n {
            dp[i][j][i][j] = d[j];
        }
    }

    let mut cnt = vec![0; n * n + 1];
    for i1 in 0..n {
        for j1 in 0..n {
            for i2 in i1..n {
                for j2 in j1..n {
                    if i1 != i2 && j1 != j2 {
                        dp[i1][j1][i2][j2] = dp[i1][j1][i2-1][j2] + dp[i1][j1][i2][j2-1] + dp[i2][j2][i2][j2] - dp[i1][j1][i2-1][j2-1];
                    } else if j1 != j2 {
                        dp[i1][j1][i2][j2] = dp[i1][j1][i2][j2-1] + dp[i1][j2][i2][j2];
                    } else if i1 != i2 {
                        dp[i1][j1][i2][j2] = dp[i1][j1][i2-1][j2] + dp[i2][j1][i2][j2];
                    }
                    cnt[(i2-i1+1)*(j2-j1+1)] = std::cmp::max(cnt[(i2-i1+1)*(j2-j1+1)], dp[i1][j1][i2][j2]);
                }
            }
        }
    }

    let mut m = 0;
    for i in 1..n*n+1 {
        if m > cnt[i] {
            cnt[i] = m;
        } else {
            m = cnt[i];
        }
    }

    let q = read1::<usize>();
    let mut ans = "".to_string();
    for i in 0..q {
        let p = read1::<usize>();
        ans = ans + &(cnt[p].to_string());
        if i + 1 < q {
            ans += "\n";
        }
    }

    println!("{}", ans);
}