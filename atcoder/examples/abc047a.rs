fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let abc = readn::<usize>(" ");
    let a = abc[0];
    let b = abc[1];
    let c = abc[2];

    if a + b == c || b + c == a || c + a == b {
        println!("Yes");
    } else {
        println!("No");
    }
}