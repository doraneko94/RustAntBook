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
    let hw = readn::<usize>(" ");
    let h = hw[0];
    let w = hw[1];
    let aa = (0..h).map(|_| read1::<String>().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut dp1 = vec![vec![vec![vec![-1; h]; w]; h]; 2];
    let mut dp2 = vec![vec![vec![vec![-1; w]; w]; h]; 2];
    let mut ans = 17;

    for a in 0..h {
        for b in (0..w).rev() {
            if b == w-1 {
                dp1[0][a][b][a] = b as isize;
            } else {
                if aa[a][b] == aa[a][b+1] {
                    dp1[0][a][b][a] = dp1[0][a][b+1][a];
                } else {
                    dp1[0][a][b][a] = b as isize;
                }
            }
        }
    }
    
    for b in 0..w {
        for a in (0..h).rev() {
            if a == h-1 {
                dp2[0][a][b][b] = a as isize;
            } else {
                if aa[a][b] == aa[a+1][b] {
                    dp2[0][a][b][b] = dp2[0][a+1][b][b];
                } else {
                    dp2[0][a][b][b] = a as isize;
                }
            }
        }
    }

    for a in 0..h {
        for b in 0..w {
            let mut d = dp1[0][a][b][a];
            let lim = dp2[0][a][b][b] as usize;
            for c in a..h {
                if c > lim {
                    dp1[0][a][b][c] = -1;
                } else {
                    d = std::cmp::min(d, dp1[0][c][b][c]);
                    dp1[0][a][b][c] = d;
                }
            }
        }
    }
    for b in 0..w {
        for a in 0..h {
            let mut c = dp2[0][a][b][b];
            let lim = dp1[0][a][b][a] as usize;
            for d in b..w {
                if d > lim {
                    dp2[0][a][b][d] = -1;
                } else {
                    c = std::cmp::min(c, dp2[0][a][d][d]);
                    dp2[0][a][b][d] = c;
                }
            }
        }
    }

    if dp1[0][0][0][h-1] == (w-1) as isize || dp2[0][0][0][w-1] == (h-1) as isize {
        println!("0");
        return;
    }
    
    for k in 1..17 {
        for a in 0..h {
            for b in 0..w {
                for c in 0..h {
                    if dp1[(k-1)%2][a][b][c] >= 0 {
                        if dp1[(k-1)%2][a][b][c] + 1 < w as isize {
                            dp1[k%2][a][b][c] = std::cmp::max(dp1[(k-1)%2][a][b][c], dp1[(k-1)%2][a][(dp1[(k-1)%2][a][b][c] + 1) as usize][c]);
                        } else {
                            dp1[k%2][a][b][c] = dp1[(k-1)%2][a][b][c];
                        }
                    }
                }
            }
        }
        for a in 0..h {
            for b in 0..w {
                for d in 0..w {
                    if dp2[(k-1)%2][a][b][d] >= 0 {
                        if dp2[(k-1)%2][a][b][d] + 1 < h as isize {
                            dp2[k%2][a][b][d] = std::cmp::max(dp2[(k-1)%2][a][b][d], dp2[(k-1)%2][(dp2[(k-1)%2][a][b][d] + 1) as usize][b][d]);
                        } else {
                            dp2[k%2][a][b][d] = dp2[(k-1)%2][a][b][d];
                        }
                    }
                }
            }
        }
        for a in 0..h {
            for b in 0..w {
                let mut d = (w-1) as isize ;
                for c in 0..h {
                    while d >= 0 && dp2[k%2][a][b][d as usize] < c as isize {
                        d -= 1;
                    }
                    if d >= 0 {
                        dp1[k%2][a][b][c] = std::cmp::max(dp1[k%2][a][b][c], d);
                    }
                }
            }
        }
        for a in 0..h {
            for b in 0..w {
                let mut c = (h-1) as isize ;
                for d in 0..w {
                    while c >= 0 && dp1[k%2][a][b][c as usize] < d as isize {
                        c -= 1;
                    }
                    if c >= 0 {
                        dp2[k%2][a][b][d] = std::cmp::max(dp2[k%2][a][b][d], c);
                    }
                }
            }
        }
        if dp1[k%2][0][0][h-1] == (w-1) as isize || dp2[k%2][0][0][w-1] == (h-1) as isize {
            ans = k;
            break;
        }
    }
    println!("{}", ans);
}