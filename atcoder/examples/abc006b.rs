pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

const MOD: usize = 10007;

fn main() {
    let n = read1::<usize>();
    let mut ans = vec![0; std::cmp::max(n+1, 4)];
    ans[1] = 0;
    ans[2] = 0;
    ans[3] = 1;

    if n >= 4 {
        for i in 4..n+1 {
            ans[i] = (ans[i-1] + ans[i-2] + ans[i-3]) % MOD;
        }
    }

    println!("{}", ans[n]);
}