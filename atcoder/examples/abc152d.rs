fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let mut ans = 0;
    let mut c = vec![vec![0; 10]; 10];

    for i in 1..n+1 {
        let k2 = i % 10;
        let mut k1 = i;
        while k1 >= 10 {
            k1 /= 10;
        }
        c[k1][k2] += 1;
    }
    for i in 1..10 {
        for j in 1..10 {
            ans += c[i][j] * c[j][i];
        }
    }

    println!("{}", ans);
}