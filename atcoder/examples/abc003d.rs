fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const MOD: usize = 1_000_000_007;

fn com(n: isize, k: isize, fac: &Vec<usize>, finv: &Vec<usize>) -> usize {
    if n < k {
        0
    } else if n < 0 || k < 0 {
        0
    } else {
        let n = n as usize;
        let k = k as usize;
        fac[n] * (finv[k] * finv[n - k] % MOD) % MOD
    }
}

fn main() {
    let rc = readn::<usize>(" ");
    let r = rc[0];
    let c = rc[1];
    let xy = readn::<usize>(" ");
    let x = xy[0];
    let y = xy[1];
    let dl = readn::<isize>(" ");
    let d = dl[0];
    let l = dl[1];

    let room = (r - x + 1) * (c - y + 1);
    let mut fac = vec![0; x*y+1];
    let mut finv = vec![0; x*y+1];
    let mut inv = vec![0; x*y+1];
    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;
    for i in 2..x*y+1 {
        fac[i] = fac[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
        finv[i] = finv[i - 1] * inv[i] % MOD;
    }

    let x = x as isize;
    let y = y as isize;
    let a = com(x*y, d, &fac, &finv) * com(x*y-d, l, &fac, &finv) % MOD;
    let a0 = com((x-1)*y, d, &fac, &finv) * com((x-1)*y-d, l, &fac, &finv) % MOD;
    let a1 = com(x*(y-1), d, &fac, &finv) * com(x*(y-1)-d, l, &fac, &finv) % MOD;
    let a2 = com((x-1)*y, d, &fac, &finv) * com((x-1)*y-d, l, &fac, &finv) % MOD;
    let a3 = com(x*(y-1), d, &fac, &finv) * com(x*(y-1)-d, l, &fac, &finv) % MOD;
    let a01 = if x > 1 && y > 1 {
        com((x-1)*(y-1), d, &fac, &finv) * com((x-1)*(y-1)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a02 = com((x-2)*y, d, &fac, &finv) * com((x-2)*y-d, l, &fac, &finv) % MOD;
    let a03 = if x > 1 && y > 1 {
        com((x-1)*(y-1), d, &fac, &finv) * com((x-1)*(y-1)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a12 = if x > 1 && y > 1 {
        com((x-1)*(y-1), d, &fac, &finv) * com((x-1)*(y-1)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a13 = com(x*(y-2), d, &fac, &finv) * com(x*(y-2)-d, l, &fac, &finv) % MOD;
    let a23 = if x > 1 && y > 1 {
        com((x-1)*(y-1), d, &fac, &finv) * com((x-1)*(y-1)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a012 = if x > 2 && y > 1 {
        com((x-2)*(y-1), d, &fac, &finv) * com((x-2)*(y-1)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a013 = if x > 1 && y > 2 {
        com((x-1)*(y-2), d, &fac, &finv) * com((x-1)*(y-2)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a023 = if x > 2 && y > 1 {
        com((x-2)*(y-1), d, &fac, &finv) * com((x-2)*(y-1)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a123 = if x > 1 && y > 2 {
        com((x-1)*(y-2), d, &fac, &finv) * com((x-1)*(y-2)-d, l, &fac, &finv) % MOD
    } else {
        0
    };
    let a0123 = if x > 2 && y > 2 {
        com((x-2)*(y-2), d, &fac, &finv) * com((x-2)*(y-2)-d, l, &fac, &finv) % MOD
    } else {
        0
    };

    let mut ans = a;
    ans = (ans + MOD - a0) % MOD;
    ans = (ans + MOD - a1) % MOD;
    ans = (ans + MOD - a2) % MOD;
    ans = (ans + MOD - a3) % MOD;
    ans = (ans + a01) % MOD;
    ans = (ans + a02) % MOD;
    ans = (ans + a03) % MOD;
    ans = (ans + a12) % MOD;
    ans = (ans + a13) % MOD;
    ans = (ans + a23) % MOD;
    ans = (ans + MOD - a012) % MOD;
    ans = (ans + MOD - a013) % MOD;
    ans = (ans + MOD - a023) % MOD;
    ans = (ans + MOD - a123) % MOD;
    ans = (ans + a0123) % MOD;
    ans = (ans * room) % MOD;
    println!("{}", ans);
}