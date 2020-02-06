use std::collections::VecDeque;

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
    let hwn = readn::<usize>(" ");
    let h = hwn[0];
    let w = hwn[1];
    let n = hwn[2];

    let mut c = Vec::with_capacity(h);
    for _ in 0..h {
        c.push(read1::<String>().chars().collect::<Vec<char>>());
    }
    let mut p = vec![(0, 0); n+1];
    for y in 0..h {
        for x in 0..w {
            if c[y][x] == 'S' {
                p[0] = (y, x);
            }
            else if c[y][x] != 'X' && c[y][x] != '.' {
                p[c[y][x] as usize - 48] = (y, x);
            }
        }
    }

    let mut sy = p[0].0;
    let mut sx = p[0].1;
    let mut ans = 0;
    for g in 1..n+1 {
        let mut q = VecDeque::new();
        q.push_back((0, sy, sx));
        let mut cc = c.clone();
        cc[sy][sx] = 'X';
        let gy = p[g].0;
        let gx = p[g].1;

        loop {
            let tyx = q.pop_front().unwrap();
            let t = tyx.0;
            let y = tyx.1;
            let x = tyx.2;

            if y == gy && x == gx {
                ans += t;
                sy = gy;
                sx = gx;
                break;
            }

            if y + 1 < h && cc[y + 1][x] != 'X' {
                q.push_back((t + 1, y + 1, x));
                cc[y + 1][x] = 'X';
            }
            if y >= 1 && cc[y - 1][x] != 'X' {
                q.push_back((t + 1, y - 1, x));
                cc[y - 1][x] = 'X';
            }
            if x + 1 < w && cc[y][x + 1] != 'X' {
                q.push_back((t + 1, y, x + 1));
                cc[y][x + 1] = 'X';
            }
            if x >= 1 && cc[y][x - 1] != 'X' {
                q.push_back((t + 1, y, x - 1));
                cc[y][x - 1] = 'X';
            }
        }
    }
    println!("{}", ans);
}