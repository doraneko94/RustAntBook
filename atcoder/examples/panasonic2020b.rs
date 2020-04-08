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
    let hw = readn::<usize>(" ");
    let h = hw[0];
    let w = hw[1];

    if h == 1 || w == 1 {
        println!("1");
    } else {
        let col0 = (h + 2 - 1) / 2;
        let col1 = h / 2;
        let n_col0 = (w + 2 - 1) / 2;
        let n_col1 = w / 2;
        println!("{}", col0 * n_col0 + col1 * n_col1);
    }
}