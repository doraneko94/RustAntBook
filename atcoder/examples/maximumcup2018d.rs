pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 10000000;

fn main() {
    let nmlx = readn::<usize>(" ");
    let n = nmlx[0];
    let m = nmlx[1];
    let l = nmlx[2];
    let x = nmlx[3];
    let a = readn::<usize>(" ");

    let mut dp = vec![vec![INF; m]; n+1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..m {
            let mut circ = a[i] / m;
            let prog = a[i] % m;
            let pre;
            if prog > j {
                pre = j + m - prog;
                circ += 1;
            } else {
                pre = j - prog;
            }
            dp[i+1][j] = dp[i][j];
            if dp[i][pre] + circ <= x {
                dp[i+1][j] = std::cmp::min(dp[i+1][j], dp[i][pre] + circ);
            }
        }
    }

    if dp[n][l] <= x {
        println!("Yes");
    } else {
        println!("No");
    }
}