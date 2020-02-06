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
    let mut c = Vec::with_capacity(h);
    for _ in 0..h {
        c.push(read1::<String>().chars().collect::<Vec<char>>());
    }

    for y in 0..h {
        for x in 0..w {
            if c[y][x] == 's' {
                if dfs(y, x, h, w, &mut c) {
                    println!("Yes");
                } else {
                    println!("No");
                }
                return;
            }
        }
    }
}

fn dfs(y: usize, x: usize, h: usize, w: usize, c: &mut [Vec<char>]) -> bool {
    if c[y][x] == 'g' {
        return true;
    }
    c[y][x] = '#';
    let mut ans = false;

    if y + 1 < h && (c[y + 1][x] == '.' || c[y + 1][x] == 'g') {
        ans = ans || dfs(y + 1, x, h, w, c);
    }

    if y >= 1 && (c[y - 1][x] == '.' || c[y - 1][x] == 'g') {
        ans = ans || dfs(y - 1, x, h, w, c);
    }

    if x + 1 < w && (c[y][x + 1] == '.' || c[y][x + 1] == 'g') {
        ans = ans || dfs(y, x + 1, h, w, c);
    }

    if x >= 1 && (c[y][x - 1] == '.' || c[y][x - 1] == 'g') {
        ans = ans || dfs(y, x - 1, h, w, c);
    }

    return ans;
}