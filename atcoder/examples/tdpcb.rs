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
    let ab = readn::<usize>(" ");
    let a = ab[0];
    let b = ab[1];

    let an = readn::<isize>(" ");
    let bn = readn::<isize>(" ");

    let mut dp = vec![vec![0; b+2]; a+2];
    for i in (0..a+1).rev() {
        for j in (0..b+1).rev() {
            if (i + j) % 2 == 0 {
                let ai = {
                    if i < a {
                        an[i]
                    } else {
                        0
                    }
                };
                let bj = {
                    if j < b {
                        bn[j]
                    } else {
                        0
                    }
                };
                dp[i][j] = std::cmp::max(dp[i+1][j] + ai, dp[i][j+1] + bj);
            } else {
                if i == a {
                    dp[i][j] = dp[i][j+1];
                } else if j == b {
                    dp[i][j] = dp[i+1][j];
                } else {
                    dp[i][j] = std::cmp::min(dp[i][j+1], dp[i+1][j]);
                }
            }
        }
    }
    

    println!("{}", dp[0][0]);
}