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
    let w = read1::<usize>();
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];
    let ab = (0..n).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let mut dp = vec![vec![vec![0; k+1]; w+1]; n+1];
    for ni in 1..n+1 {
        for wi in 1..w+1 {
            for ki in 1..k+1 {
                if wi >= ab[ni-1][0] && ki >= 1 {
                    dp[ni][wi][ki] = std::cmp::max(dp[ni-1][wi-ab[ni-1][0]][ki-1] + ab[ni-1][1], dp[ni-1][wi][ki]);
                } else {
                    dp[ni][wi][ki] = dp[ni-1][wi][ki];
                }
            }
        }
    }
    println!("{}", dp[n][w][k]);
}