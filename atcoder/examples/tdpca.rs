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
    let p = readn::<usize>(" ");
    let mut dp = vec![vec![0; 10001]; n+1];

    dp[0][0] = 1;
    for i in 1..n+1 {
        for j in 0..10001 {
            if j >= p[i-1] {
                dp[i][j] = std::cmp::max(dp[i-1][j], dp[i-1][j-p[i-1]]);
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }

    let mut ans = 0;
    for j in 0..10001 {
        ans += dp[n][j];
    }

    println!("{}", ans);
}