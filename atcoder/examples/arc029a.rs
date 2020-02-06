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
    let mut t = Vec::with_capacity(n);

    for _ in 0..n {
        t.push(read1::<usize>());
    }

    let mut ans = t.iter().sum::<usize>();
    for i in 0..2_usize.pow(n as u32) {
        let mut g1 = 0;
        let mut g2 = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                g1 += t[j];
            } else {
                g2 += t[j];
            }
        }
        ans = std::cmp::min(ans, std::cmp::max(g1, g2));
    }

    println!("{}", ans);
}