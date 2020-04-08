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
    let n = read1::<usize>();
    let a = readn::<isize>(" ");

    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    let mut tail = -1;
    while r < n {
        if tail < a[r] {
            ans += r - l + 1;
            tail = a[r];
            r += 1;
        } else {
            l = r;
            tail = -1;
        }
    }

    println!("{}", ans);
}