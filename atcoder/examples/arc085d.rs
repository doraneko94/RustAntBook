fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let nzw = readn::<usize>(" ");
    let n = nzw[0];
    let _z = nzw[1];
    let w = nzw[2];

    let a = readn::<isize>(" ");
    let mut dp = vec![vec![0; n]; 2];
    dp[0][n-1] = (a[n-1] - w as isize).abs();
    if n > 1 {
        dp[1][n-1] = (a[n-2] - a[n-1]).abs();
    }
    for i in (0..n-1).rev() {
        if i > 0 {
            dp[1][i] = std::cmp::min((a[i-1]-a[n-1]).abs(), dp[0][i+1]);
        }
        dp[0][i] = std::cmp::max(dp[0][i+1], dp[1][i+1]);
    }
    //dp[0][0] = std::cmp::max(dp[0][1], dp[1][1]);
    println!("{}", dp[0][0]);
    //dp[0][n-1] = (a[n-2] - a[n-1]).abs();
    //dp[0][n-2] = (a[n-3] - a[n-1]).abs(), (a[n-2]-a[n-1]).abs()
}