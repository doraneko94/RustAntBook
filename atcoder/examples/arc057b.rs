pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize =  100000000000;

fn main() {
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];
    let mut dp = vec![vec![INF; n+1]; n+1];
    for i in 0..n+1 {
        dp[i][0] = 0;
    }
    let mut total = 0;

    for i in 1..n+1 {
        let a = read1::<usize>();
        let total_new = total + a;
        for j in 1..n+1 {
            let mut require = INF;
            if dp[i-1][j-1] != INF {
                if total > 0 {
                    require =  dp[i-1][j-1] * (total_new - total) / total + 1;
                } else {
                    require = 1;
                }
                if require > a {
                    require = INF;
                }
            }
            dp[i][j] = std::cmp::min(dp[i-1][j], dp[i-1][j-1] + require);
        }
        total = total_new;
    }

    let mut ans = 0;
    for j in 0..n+1 {
        if dp[n][j] > k {
            break;
        }
        ans = j;
    }
    if total == k { println!("1"); }
    else { println!("{}", ans); }
}