pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const MOD: usize = 1000000007;

fn main() {
    let d = read1::<usize>();
    let n = read1::<String>().chars().map(|c| c as usize - 48).collect::<Vec<usize>>();
    let l = n.len();
    let mut dp = vec![vec![vec![0; 2]; d]; l+1];
    dp[0][0][1] = 1;
    for i in 0..n.len() {
        for j in 0..d {
            for k in 0..10 {
                dp[i+1][(j+k)%d][0] += dp[i][j][0] % MOD;
                dp[i+1][(j+k)%d][0] %= MOD;
            }
            for k in 0..n[i] {
                dp[i+1][(j+k)%d][0] += dp[i][j][1] % MOD;
                dp[i+1][(j+k)%d][0] %= MOD;
            }
            dp[i+1][(j+n[i])%d][1] += dp[i][j][1] % MOD;
            dp[i+1][(j+n[i])%d][1] %= MOD;
        }
    }
    println!("{}", dp[l][0][0] + dp[l][0][1] - 1);
}