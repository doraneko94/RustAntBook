fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1_000_000_000;

fn main() {
    let n = read1::<String>().chars().collect::<Vec<char>>();
    let l = n.len();
    let mut dp = vec![vec![INF; l+1]; 2];
    dp[0][0] = 0;
    dp[1][0] = INF;

    for i in 0..l {
        let num = n[i] as usize - 48;
        dp[0][i+1] = std::cmp::min(dp[0][i] + num, dp[1][i] + num);
        dp[1][i+1] = std::cmp::min(dp[0][i] + 1 + 10 - num, dp[1][i] + 9 - num);
    }

    println!("{}", std::cmp::min(dp[0][l], dp[1][l]));
}