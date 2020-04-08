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
    let p = readn::<usize>(" ");

    let mut r = vec![0; n+1];
    for i in 1..n+1 {
        r[p[i-1]] = i;
    }

    let mut a = "".to_string();
    let mut b = "".to_string();
    for i in 1..n+1 {
        a = a + &((30000 * i).to_string()) + " ";
        b = b + &((30000 * (n - i) + r[i]).to_string()) + " ";
    }

    println!("{}", a);
    println!("{}", b);
}