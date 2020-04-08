fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];

    let mut s = Vec::with_capacity(n);
    let mut flg = false;
    for _ in 0..n {
        let ss = read1::<usize>();
        if ss == 0 {
            flg = true;
        }
        s.push(ss);
    }
    if flg {
        println!("{}", n);
        return;
    }

    let mut ans = 0usize;
    let mut l = 0;
    let mut r = 0;
    let mut num = 1;
    for l in 0..n {
        while r < n && num * s[r] <= k {
            num *= s[r];
            r += 1;
        }
        ans = std::cmp::max(ans, r - l);
        if l == r {
            r += 1;
        } else {
            num /= s[l];
        }
    }

    println!("{}", ans);
}