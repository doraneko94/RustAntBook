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
    let abx = readn::<usize>(" ");
    let a = abx[0];
    let b = abx[1];
    let x = abx[2];

    let s = (a + x - 1) / x;
    let e = b / x;
    println!("{}", e + 1 - s);
}