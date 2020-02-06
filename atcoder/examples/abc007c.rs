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
    let rc = readn::<usize>(" ");
    let r = rc[0];
    let c = rc[1];
    let syx = readn::<usize>(" ");
    let sy = syx[0] - 1;
    let sx = syx[1] - 1;
    let gyx = readn::<usize>(" ");
    let gy = gyx[0] - 1;
    let gx = gyx[1] - 1;

    let mut map = Vec::with_capacity(r);
    for _ in 0..r {
        map.push(read1::<String>().chars().collect::<Vec<char>>());
    }

    let mut q = VecDeque::new();
    q.push_back((0_usize, sy, sx));
    map[sy][sx] = '#';

    loop {
        let tuple = q.pop_front().unwrap();
        let t = tuple.0;
        let y = tuple.1;
        let x = tuple.2;

        if y == gy && x == gx {
            println!("{}", t);
            return;
        }

        if y + 1 < r && map[y + 1][x] != '#' {
            q.push_back((t + 1, y + 1, x));
            map[y + 1][x] = '#';
        }
        if y >= 1 && map[y - 1][x] != '#' {
            q.push_back((t + 1, y - 1, x));
            map[y - 1][x] = '#';
        }
        if x + 1 < c && map[y][x + 1] != '#' {
            q.push_back((t + 1, y, x + 1));
            map[y][x + 1] = '#';
        }
        if x >= 1 && map[y][x - 1] != '#' {
            q.push_back((t + 1, y, x - 1));
            map[y][x - 1] = '#';
        }
    }
}

