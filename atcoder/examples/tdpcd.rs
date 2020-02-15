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

fn main() {
    let nd = readn::<usize>(" ");
    let n = nd[0];
    let mut d = nd[1];
    let factors = [2, 3, 5];
    let mut v = vec![0, 0, 0];
    for i in 0..3 {
        let factor = factors[i];
        loop {
            if d % factor == 0 {
                d /= factor;
                v[i] += 1;
            } else {
                break;
            }
        }
    }
    if d != 1 {
        println!("0");
        return;
    }
    let mut dp = vec![vec![vec![vec![0.0; v[2]+1]; v[1]+1]; v[0]+1]; n+1];
    dp[0][0][0][0] = 1.0;
    for i in 0..n {
        for x in 0..v[0]+1 {
            for y in 0..v[1]+1 {
                for z in 0..v[2]+1 {
                    dp[i+1][x][y][z] += dp[i][x][y][z] / 6.0;
                    dp[i+1][min(v[0], x + 1)][y][z] += dp[i][x][y][z] / 6.0; 
                    dp[i+1][x][min(v[1], y + 1)][z] += dp[i][x][y][z] / 6.0;
                    dp[i+1][min(v[0], x + 2)][y][z] += dp[i][x][y][z] / 6.0;
                    dp[i+1][x][y][min(v[2], z + 1)] += dp[i][x][y][z] / 6.0;
                    dp[i+1][min(v[0], x + 1)][min(v[1], y + 1)][z] += dp[i][x][y][z] / 6.0;
                }
            }
        }
    }
    println!("{:.09}", dp[n][v[0]][v[1]][v[2]]);
}