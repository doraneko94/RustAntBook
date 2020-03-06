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
    let d = read1::<usize>();
    let n = read1::<usize>();
    let m = read1::<usize>();
    let mut shop = Vec::with_capacity(n+1);
    shop.push(0);
    for _ in 0..n-1 {
        shop.push(read1::<usize>());
    }
    shop.push(d);
    shop.sort();

    let home = (0..m).map(|_| read1::<usize>()).collect::<Vec<usize>>();

    let mut ans = 0;
    for h in home.iter() {
        ans += match shop.binary_search(h) {
            Ok(_) => 0,
            Err(i) => std::cmp::min(*h - shop[i-1], shop[i] - *h),
        };
    }

    println!("{}", ans);
}