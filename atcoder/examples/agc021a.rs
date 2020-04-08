fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn dfs(s: usize, k: usize, nine: bool, n: &Vec<char>) -> usize {
    let mut s = s;
    if nine {
        s += 9;
    } else {
        s += (n[k] as usize) - 48;
    }
    if k == n.len() - 1 {
        return s;
    }

    if !nine {
        std::cmp::max(dfs(s, k+1, false, n), dfs(s-1, k+1, true, n))
    } else {
        std::cmp::max(dfs(s, k+1, true, n), dfs(s-1, k+1, true, n))
    }
}

fn main() {
    let n = read1::<String>().chars().collect::<Vec<char>>();
    println!("{}", dfs(0, 0, false, &n));    
}