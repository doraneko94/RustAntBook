use std::cmp::min;

pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1000000;

fn main() {
    let s1 = read1::<String>().chars().collect::<Vec<char>>();
    let s2 = read1::<String>().chars().collect::<Vec<char>>();
    let l1 = s1.len();
    let l2 = s2.len();

    let mut dp = vec![vec![INF; l2 + 1]; l1 + 1];
    for i in 0..l1+1 {
        dp[i][0] = i;
    }
    for i in 0..l2+1 {
        dp[0][i] = i;
    }
    for i in 0..l1 {
        for j in 0..l2 {
            let cost;
            if s1[i] == s2[j] { cost = 0 }
            else { cost = 1 }

            dp[i+1][j+1] = min(dp[i][j+1] + 1, min(dp[i+1][j] + 1, dp[i][j] + cost));
        }
    }
    println!("{}", dp[l1][l2]);
}