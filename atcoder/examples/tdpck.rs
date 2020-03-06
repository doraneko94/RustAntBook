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
    let n = read1::<usize>();
    let mut dp = Vec::new();
    let mut rl = Vec::with_capacity(n);
    for _ in 0..n {
        let xr = readn::<isize>(" ");
        let l = xr[0] - xr[1];
        let r = xr[0] + xr[1];
        rl.push((r, l));
    }
    rl.sort();

    for &(_, l) in rl.iter() {
        let revl = -1 * (l as isize);
        let index = match dp.binary_search(&revl) {
            Ok(e) => e,
            Err(e) => e,
        };
        if index == dp.len() { dp.push(revl); }
        else { dp[index] = revl; }
    }

    println!("{}", dp.len());
}