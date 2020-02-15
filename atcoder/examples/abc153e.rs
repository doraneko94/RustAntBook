pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1000000000;

fn main() {
    let hn = readn::<usize>(" ");
    let h = hn[0];
    let n = hn[1];

    let mut dp = vec![vec![INF; h+1]; n+1];
    for i in 0..n+1 {
        dp[i][0] = 0;
    }

    for i in 1..n+1 {
        let ab = readn::<usize>(" ");
        for j in 1..h+1 {
            if j >= ab[0] {
                dp[i][j] = std::cmp::min(dp[i-1][j], dp[i][j-ab[0]] + ab[1]);
            } else {
                dp[i][j] = std::cmp::min(dp[i-1][j], dp[i][0] + ab[1]);
            }
        }
    }
    println!("{}", dp[n][h]);
}