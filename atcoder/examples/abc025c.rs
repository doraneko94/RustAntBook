use std::collections::HashMap;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn calc(x: usize, b: &Vec<Vec<usize>>, c: &Vec<Vec<usize>>, mem: &mut HashMap<usize, usize>) -> usize {
    match mem.get(&x) {
        Some(&e) => { return e;},
        None => {
            let mut e = 0;
            for i in 0..2 {
                for j in 0..3 {
                    if (x >> (i * 3 + j) & 1) == (x >> ((i + 1) * 3 + j) & 1) {
                        e += b[i][j];
                    }
                }
            }
            for i in 0..3 {
                for j in 0..2 {
                    if (x >> (i * 3 + j) & 1) == (x >> (i * 3 + (j+1)) & 1) {
                        e += c[i][j];
                    }
                }
            }
            mem.insert(x, e);
            return e;
        },
    };
}

fn tree(x: usize, b: &Vec<Vec<usize>>, c: &Vec<Vec<usize>>, mem: &mut HashMap<usize, usize>,
        t: usize) -> usize {
    match mem.get(&x) {
        Some(&e) => { return e; },
        None => {
            if t == 8 {
                let v = 1+2+4+8+16+32+64+128+256;
                return calc(x | (v << 9), b, c, mem);
            }
            let v = x >> 9;
            let mut ret = {
                if t % 2 == 0 {
                    0
                } else {
                    1_000_000
                }
            };
            for i in 0..9 {
                if v >> i & 1 == 0 {
                    if t % 2 == 0 {
                        ret = std::cmp::max(ret, tree(x | (1 << i + 9), b, c, mem, t+1));
                    } else {
                        ret = std::cmp::min(ret, tree(x | (1 << i) | (1 << i + 9), b, c, mem, t+1));
                    }
                    
                }
            }
            mem.insert(x, ret);
            return ret;
        }
    };
}

fn main() {
    let b = (0..2).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let c = (0..3).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let all = b.iter().map(|v| v.iter().sum::<usize>()).sum::<usize>() + c.iter().map(|v| v.iter().sum::<usize>()).sum::<usize>();

    let mut mem = HashMap::new();
    let ans = tree(0, &b, &c, &mut mem, 0);
    println!("{}", ans);
    println!("{}", all - ans);
}