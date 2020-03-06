use std::collections::HashMap;

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
    let nm = readn::<isize>(" ");
    let mut n = nm[0].abs() as usize;
    let m = nm[1] as usize;

    let mut k = 2;
    let mut dict = HashMap::new();
    while n > 1 {
        if (n as f64).sqrt() as usize + 1 < k {
            let count = dict.entry(n).or_insert(0);
            *count += 1;
            break;
        }
        while n % k == 0 {
            let count = dict.entry(k).or_insert(0);
            *count += 1;
            n /= k;
        }
        k += 1;
    }
    let mut ans = 1;
    let mut fac = vec![1; 200000];
    let mut finv = vec![1; 200000];
    let mut inv = vec![1; 200000];
    for i in 2..200000 {
        fac[i] = fac[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
        finv[i] = finv[i - 1] * inv[i] % MOD;
    }

    for (_, &v) in dict.iter() {
        ans = ans * (fac[m+v-1] * (finv[v] * finv[m-1] % MOD) % MOD) % MOD;
    }
    
    for _ in 0..m-1 {
        ans = ans * 2 % MOD;
    }
    
    println!("{}", ans);
}