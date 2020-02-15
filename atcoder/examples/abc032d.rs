pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 10000000000;

fn main() {
    let nw = readn::<usize>(" ");
    let n = nw[0];
    let w = nw[1];
    let vw = (0..n).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let v_max = vw.iter().map(|e| e[0]).fold(0, |m, v| std::cmp::max(m, v));

    if v_max <= 1000 {
        let v_sum = vw.iter().map(|e| e[0]).sum::<usize>();
        let mut dp = vec![vec![INF; v_sum+1]; n+1];
        dp[0][0] = 0;
        for i in 0..n {
            for j in 0..v_sum+1 {
                if vw[i][0] > j {
                    dp[i+1][j] = std::cmp::min(dp[i][j], dp[i][0] + vw[i][1]);
                } else {
                    dp[i+1][j] = std::cmp::min(dp[i][j], dp[i][j-vw[i][0]] + vw[i][1]);
                }
            }
        }
        let mut ans = 0;
        for i in 0..v_sum+1 {
            if dp[n][i] > w {
                break;
            }
            ans = i;
        }
        println!("{}", ans);
    } 
    
    else if n <= 30 {
        let mut ans = 0;
        let half = n / 2;
        let half2 = n - half;
        let mut left = Vec::with_capacity(2_usize.pow(half as u32));
        for i in 0..2_usize.pow(half as u32) {
            let mut w_sum = 0;
            let mut v_sum = 0;
            for j in 0..half {
                if i>>j & 1 == 1 {
                    v_sum += vw[j][0];
                    w_sum += vw[j][1];
                }
            }
            left.push(vec![v_sum, w_sum]);
        }
        left.sort_by(|a, b| a[1].cmp(&b[1]));
        for i in 1..left.len() {
            left[i][0] = std::cmp::max(left[i-1][0], left[i][0]);
        }

        //let mut right = Vec::with_capacity(2_usize.pow(half2 as u32));
        for i in 0..2_usize.pow(half2 as u32) {
            let mut w_sum = 0;
            let mut v_sum = 0;
            for j in 0..half2 {
                if i>>j & 1 == 1 {
                    v_sum += vw[j+half][0];
                    w_sum += vw[j+half][1];
                }
            }
            if w_sum <= w {
                let w_rest = w - w_sum;
                let i_max = match left.binary_search_by(|e| e[1].cmp(&(w_rest + 1))) {
                    Ok(e) => e - 1,
                    Err(e) => e - 1,
                };
                ans = std::cmp::max(ans, left[i_max][0] + v_sum);
            }
        }

        println!("{}", ans);
    }

    else {
        let mut dp = vec![vec![0; w+1]; n+1];
        for i in 0..n {
            for j in 0..w+1 {
                if j >= vw[i][1] {
                    dp[i+1][j] = std::cmp::max(dp[i][j], dp[i][j-vw[i][1]] + vw[i][0]);
                } else {
                    dp[i+1][j] = dp[i][j];
                }
            }
        }
        let ans = dp[n].iter().map(|&e| e).fold(0, |m, v| std::cmp::max(m, v));
        println!("{}", ans);
    }
}