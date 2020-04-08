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
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];

    let mut l = readn::<usize>(" ");
    l.sort_by(|a, b| b.cmp(&a));

    let mut sum = 0;
    for i in 0..k {
        sum += l[i];
    }

    println!("{}", sum);
}