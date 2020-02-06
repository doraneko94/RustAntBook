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
    let hw = readn::<usize>(" ");
    let h = hw[0];
    let w = hw[1];
    let mut c = (0..h).map(|_| 
        read1::<String>().chars().collect::<Vec<char>>()
    ).collect::<Vec<Vec<char>>>();

    let mut sy = 0;
    let mut sx = 0;
    let mut gy = 0;
    let mut gx = 0;
    for y in 0..h {
        for x in 0..w {
            if c[y][x] == 's' {
                sy = y;
                sx = x;
            }
            if c[y][x] == 'g' {
                gy = y;
                gx = x;
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back((0, sy, sx));
    c[sy][sx] = '@';

    loop {
        let tyx = q.pop_front().unwrap();
        let t = tyx.0;
        let y = tyx.1;
        let x = tyx.2;

        if t > 2 {
            println!("NO");
            return;
        }

        if y == gy && x == gx {
            println!("YES");
            return;
        }

        if y + 1 < h && c[y + 1][x] != '@' {
            if c[y + 1][x] == '#' {
                q.push_back((t + 1, y + 1, x));
            } else {
                q.push_front((t, y + 1, x));
            }
            c[y + 1][x] = '@';
        }
        if y >= 1 && c[y - 1][x] != '@' {
            if c[y - 1][x] == '#' {
                q.push_back((t + 1, y - 1, x));
            } else {
                q.push_front((t, y - 1, x));
            }
            c[y - 1][x] = '@';
        }
        if x + 1 < w && c[y][x + 1] != '@' {
            if c[y][x + 1] == '#' {
                q.push_back((t + 1, y, x + 1));
            } else {
                q.push_front((t, y, x + 1));
            }
            c[y][x + 1] = '@';
        }
        if x >= 1 && c[y][x - 1] != '@' {
            if c[y][x - 1] == '#' {
                q.push_back((t + 1, y, x - 1));
            } else {
                q.push_front((t, y, x - 1));
            }
            c[y][x - 1] = '@';
        }
    }

}