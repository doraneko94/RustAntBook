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
    let st = readn::<isize>(" ");
    let sx = st[0];
    let sy = st[1];
    let tx = st[2];
    let ty = st[3];

    let mut ans = "".to_string();
    for _ in 0..(ty-sy) {
        ans += "U";
    }
    for _ in 0..(tx-sx) {
        ans += "R";
    }
    for _ in 0..(ty-sy) {
        ans += "D";
    }
    for _ in 0..(tx-sx) {
        ans += "L";
    }
    ans += "L";
    for _ in 0..(ty-sy+1) {
        ans += "U";
    }
    for _ in 0..(tx-sx+1) {
        ans += "R";
    }
    ans += "D";
    ans += "R";
    for _ in 0..(ty-sy+1) {
        ans += "D";
    }
    for _ in 0..(tx-sx+1) {
        ans += "L";
    }
    ans += "U";

    println!("{}", ans);
}