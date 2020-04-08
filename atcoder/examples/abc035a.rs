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
    let wh = readn::<usize>(" ");
    let w = wh[0];
    let h = wh[1];

    if w * 3 == h * 4 {
        println!("4:3");
    } else {
        println!("16:9");
    }
}