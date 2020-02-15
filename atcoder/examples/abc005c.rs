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
    let t = read1::<usize>();
    let n = read1::<usize>();
    let a = readn::<usize>(" ");
    let m = read1::<usize>();
    let b = readn::<usize>(" ");
    let mut ai = 0;
    let mut bi = 0;

    while ai < n && bi < m {
        if a[ai] <= b[bi] && a[ai] + t >= b[bi] {
            bi += 1;
        }
        ai += 1;
    }

    if bi >= m {
        println!("yes");
    } else {
        println!("no");
    }
}