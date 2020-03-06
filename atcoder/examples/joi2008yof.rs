use std::cmp::min;

pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1_000_000_000_000;

fn main() {
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];

    let mut fare = vec![vec![INF; n]; n];
    for i in 0..n {
        fare[i][i] = 0;
    }
    let mut ans = String::new();
    let mut flg = false;
    for _ in 0..k {
        let q = readn::<usize>(" ");
        if q[0] == 0 {
            if flg {
                ans += "\n";
            } else {
                flg = true;
            }
            let a = q[1] - 1;
            let b = q[2] - 1;
            let f = fare[a][b];
            if f == INF {
                ans += "-1";
            } else {
                ans += &(f.to_string());
            }
            
        } else {
            let c = q[1] - 1;
            let d = q[2] - 1;
            let e = q[3];
            if fare[c][d] > e {
                fare[c][d] = e;
                fare[d][c] = e;
                for i in 0..n {
                    for j in 0..n {
                        fare[i][j] = min(min(fare[i][c] + fare[c][d] + fare[d][j],
                                             fare[i][d] + fare[d][c] + fare[c][j]),
                                         fare[i][j]);
                    }
                }
            }
        }
    }
    println!("{}", ans);
}