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

    let mut a = Vec::with_capacity(h);
    for _ in 0..h {
        a.push(read1::<String>().chars().collect::<Vec<char>>());
    }

    let mut black = 0;
    let mut q = VecDeque::new();
    for y in 0..h {
        for x in 0..w {
            if a[y][x] == '#' {
                black += 1;
                q.push_back((0, y, x));
            }
        }
    }

    if black == h * w {
        println!("0");
        return;
    }

    loop {
        let tyx = q.pop_front().unwrap();
        let t = tyx.0;
        let y = tyx.1;
        let x = tyx.2;

        if y + 1 < h && a[y + 1][x] == '.' {
            q.push_back((t + 1, y + 1, x));
            a[y + 1][x] = '#';
            black += 1;
        }
        if y >= 1 && a[y - 1][x] == '.' {
            q.push_back((t + 1, y - 1, x));
            a[y - 1][x] = '#';
            black += 1;
        }
        if x + 1 < w && a[y][x + 1] == '.' {
            q.push_back((t + 1, y, x + 1));
            a[y][x + 1] = '#';
            black += 1;
        }
        if x >= 1 && a[y][x - 1] == '.' {
            q.push_back((t + 1, y, x - 1));
            a[y][x - 1] = '#';
            black += 1;
        }

        if black == h * w {
            println!("{}", t + 1);
            return;
        }
    }
}