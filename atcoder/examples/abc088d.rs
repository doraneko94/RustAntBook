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
    let mut s = Vec::with_capacity(h);
    for _ in 0..h {
        s.push(read1::<String>().chars().collect::<Vec<char>>());
    }

    let sharp = s.iter().map(|v| v.iter().map(|&c| {
        if c == '#' { 1 }
        else { 0 }
    }).sum::<usize>()).sum::<usize>();
    
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    loop {
        if q.len() == 0 {
            println!("-1");
            return;
        }
        let tyx = q.pop_front().unwrap();
        let t = tyx.0;
        let y = tyx.1;
        let x = tyx.2;

        if y + 1 == h && x + 1 == w {
            println!("{}", h * w - sharp - t - 1);
            return;
        }

        if y + 1 < h && s[y + 1][x] == '.' {
            q.push_back((t + 1, y + 1, x));
            s[y + 1][x] = '#';
        }
        if y >= 1  && s[y - 1][x] == '.' {
            q.push_back((t + 1, y - 1, x));
            s[y - 1][x] = '#';
        }
        if x + 1 < w && s[y][x + 1] == '.' {
            q.push_back((t + 1, y, x + 1));
            s[y][x + 1] = '#';
        }
        if x >= 1  && s[y][x - 1] == '.' {
            q.push_back((t + 1, y, x - 1));
            s[y][x - 1] = '#';
        }
    }
}