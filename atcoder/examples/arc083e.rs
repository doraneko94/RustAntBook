pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn depth(pi: usize, p: &Vec<usize>) -> usize {
    if pi == 1 {
        return 1;
    }
    depth(p[pi-2], p) + 1
}

const INF: usize = 1000000;

fn main() {
    let n = read1::<usize>();
    if n == 1 {
        let _p = read1::<String>();
        let _x = read1::<String>();
        println!("POSSIBLE");
        return;
    }
    let p = readn::<usize>(" ");
    let x = readn::<usize>(" ");

    let mut c = vec![Vec::new(); n];
    for i in 0..n-1 {
        c[p[i]-1].push(i+1);
    }

    let mut d = Vec::with_capacity(n);
    d.push((0, 0));
    for i in 0..n-1 {
        d.push((depth(p[i], &p), i+1));
    }
    d.sort_by(|a, b| (b.0).cmp(&(a.0)));
    
    let mut dp = vec![INF; n];
    for i in 0..n {
        let v = d[i].1;
        if c[v].len() == 0 {
            dp[v] = 0;
        } else {
            let c_num = c[v].len();
            let mut x_max = 0;
            let mut x_min = 0;
            let mut x_delta = Vec::new();
            for j in 0..c_num {
                if dp[c[v][j]] < x[c[v][j]] {
                    x_min += dp[c[v][j]];
                    x_max += x[c[v][j]];
                    x_delta.push(x[c[v][j]] - dp[c[v][j]]);
                } else {
                    x_min += x[c[v][j]];
                    x_max += dp[c[v][j]];
                    x_delta.push(dp[c[v][j]] - x[c[v][j]]);
                }
            }
            if x_min > x[v] {
                println!("IMPOSSIBLE");
                return;
            }
            let rest = x[v] - x_min;
            let mut dp2 = vec![vec![x_max; rest+1]; c_num+1];

            for y in 0..c_num {
                for x in 0..rest {
                    dp2[y+1][x+1] = std::cmp::min(dp2[y][x+1], dp2[y+1][x]);
                    if x_delta[y] <= x + 1 {
                        dp2[y+1][x+1] = std::cmp::min(dp2[y+1][x+1], dp2[y][x+1-x_delta[y]] - x_delta[y]);
                    }
                }
            }
            dp[v] = dp2[c_num][rest];
        }
    }

    println!("POSSIBLE");
}