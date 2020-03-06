pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const MOD: usize = 1_000_000_007;

fn main() {
    let nmq = readn::<usize>(" ");
    let n = nmq[0];
    let m = nmq[1];
    let q = nmq[2];

    let a = readn::<usize>(" ");

    let mut dp = vec![vec![0; m+1]; n+1];
    dp[0][0] = 1;
    
    for i in 0..n {
        dp[i+1][0] = 1;
        for j in 1..m+1 {
            if j >= a[i] + 1 {
                dp[i+1][j] = (MOD + dp[i][j] + dp[i+1][j-1] - dp[i][j-a[i]-1]) % MOD;
            } else {
                dp[i+1][j] = (dp[i][j] + dp[i+1][j-1]) % MOD;
            }
        }
    }
    
    let mut rdp = vec![vec![0; m+1]; n];
    for i in 0..n {
        rdp[i][0] = 1;
        for j in 1..m+1 {
            if j >= a[i] + 1 {
                rdp[i][j] = (MOD + dp[n][j] + rdp[i][j-a[i]-1] - dp[n][j-1]) % MOD;
            } else {
                rdp[i][j] = (MOD + dp[n][j] - dp[n][j-1]) % MOD;
            }
        }
    }
    
    for _ in 0..q {
        let kx = readn::<usize>(" ");
        let k = kx[0];
        let x = kx[1];

        println!("{}", rdp[k-1][m-x]);
    }
}