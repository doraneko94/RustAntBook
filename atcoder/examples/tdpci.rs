fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1_000_000;

fn rec(l: usize, r: usize, dp: &mut Vec<Vec<usize>>, s: &Vec<char>) -> usize {
    if dp[l][r] != INF {
        return dp[l][r];
    }
    let mut res = 0;
    if r == l + 3 {
        if s[l] == 'i' && s[l+1] == 'w' && s[l+2] == 'i' {
            dp[l][r] = 1;
            return 1;
        }
    }

    if r >= l + 3 {
        if r >= 1 && s[l] == 'i' && s[l+1] == 'w' && s[r-1] == 'i' {
            if (r - l - 3) % 3 == 0 && rec(l+2, r-1, dp, s) == (r - l - 3) / 3 {
                res = dp[l+2][r-1] + 1;
            }
        }
        if r >= 2 && s[l] == 'i' && s[r-2] == 'w' && s[r-1] == 'i' {
            if (r - l - 3) % 3 == 0 && rec(l+1, r-2, dp, s) == (r - l - 3) / 3 {
                res = dp[l+1][r-2] + 1;
            }
        }
    }

    for mid in l+1..r {
        res = std::cmp::max(rec(l, mid, dp, s) + rec(mid, r, dp, s), res);
    }

    dp[l][r] = res;
    res
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let n = s.len();
    let mut dp = vec![vec![INF; n+1]; n+1];
    rec(0, n, &mut dp, &s);

    println!("{}", dp[0][n]);
}