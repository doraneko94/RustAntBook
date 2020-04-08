fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const MOD: usize = 1_000_000_007;

fn main() {
    let n = read1::<usize>();
    let a = readn::<usize>(" ");
    let mut c = vec![1, 0, 0];
    let mut dp = vec![0; n];
    dp[0] = 3;

    if a[0] != 0 {
        println!("0");
        return;
    }

    for i in 1..n {
        let mut k = 0;
        for j in 0..3 {
            if c[j] == a[i] {
                if k == 0 {
                    c[j] += 1;
                }
                k += 1;
            }
        }
        if k == 0 {
            println!("0");
            return;
        }
        dp[i] = dp[i-1] * k % MOD;
    }

    println!("{}", dp[n-1]);
}