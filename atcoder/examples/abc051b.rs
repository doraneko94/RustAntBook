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
    let ks = readn::<usize>(" ");
    let k = ks[0];
    let s = ks[1];

    let upper = std::cmp::min(k, s);
    let mut ans = 0;
    for x in 0..(upper+1) {
        for y in 0..(std::cmp::min(k, s - x)+1) {
            if x + y + k >= s {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}