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

fn permutation(n: usize) -> Vec<Vec<usize>> {
    let mut incl = n - 1;
    let mut temp = vec![0; n];
    let mut ret = Vec::new();

    while incl > 0 || temp[0] < n - 1 {
        if temp[incl] == n - 1 {
            temp[incl] = 0;
            incl -= 1;
            continue;
        }

        temp[incl] += 1;
        incl = n - 1;
        let mut flg = true;
        let mut test = vec![0; n];
        for &e in temp.iter() {
            if test[e] == 1 {
                flg = false;
                break;
            } else {
                test[e] = 1;
            }
        }

        if flg { ret.push(temp.clone()) }
    }

    ret
}

fn main() {
    let nmr = readn::<usize>(" ");
    let n = nmr[0];
    let m = nmr[1];
    let r = nmr[2];

    let rs = readn::<usize>(" ");
    let mut dp = vec![vec![vec![INF; n]; n]; n+1];
    for i in 0..n {
        dp[0][i][i] = 0;
    }
    for _ in 0..m {
        let abc = readn::<usize>(" ");
        let a = abc[0] - 1;
        let b = abc[1] - 1;
        let c = abc[2];
        dp[0][a][b] = c;
        dp[0][b][a] = c;
    }

    for k in 1..n+1 {
        for i in 0..n {
            for j in 0..n {
                dp[k][i][j] = std::cmp::min(dp[k-1][i][j], dp[k-1][i][k-1] + dp[k-1][k-1][j]);
            }
        }
    }

    let p = permutation(r);
    let mut minl = INF;
    for v in p.iter() {
        let mut cand = 0;
        for i in 0..r-1 {
            cand += dp[n][rs[v[i]] - 1][rs[v[i+1]] - 1];
        }
        minl = std::cmp::min(minl, cand);
    }

    println!("{}", minl);
}