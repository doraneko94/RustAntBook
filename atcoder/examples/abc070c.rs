fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    if a > b {
        return gcd(b, a % b);
    } else {
        return gcd(a, b % a);
    }
    
}

fn main() {
    let n = read1::<usize>();
    let t = (0..n).map(|_| read1::<usize>()).collect::<Vec<usize>>();
    let ans = t.iter().fold(1, |m, &a| {
        let g = gcd(a, m);
        m / g * a
    });
    println!("{}", ans);
}