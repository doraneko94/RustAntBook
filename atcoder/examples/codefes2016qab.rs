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
    let a = readn::<usize>(" ");

    let mut ans = 0;
    for i in 0..n {
        if i < a[i] - 1 && i == a[a[i] - 1] - 1 {
            ans += 1;
        }
    }
    
    println!("{}", ans);
}