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
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];
    let mut p = Vec::with_capacity(n+1);
    p.push(0);
    for _ in 0..n {
    	p.push(read1::<usize>());
    }

    let mut p34 = Vec::with_capacity((n + 1) * (n + 1));
    for &e1 in p.iter() {
        for &e2 in p.iter() {
            p34.push(e1 + e2);
        }
    }
    p34.sort();
    let mut ans = 0;
    for &e1 in p.iter() {
        for &e2 in p.iter() {
            if e1 + e2 < m {
                let bin = p34.binary_search(&(m - e1 - e2));
                let cand = match bin {
                    Ok(pos) => p34[pos],
                    Err(pos) => p34[pos-1],
                };
                ans = std::cmp::max(ans, cand + e1 + e2);
            }
        }
    }
    println!("{}", ans);
}