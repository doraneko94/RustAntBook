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
    let nm = readn::<usize>(" ");
    let _n = nm[0];
    let m = nm[1];

    let mut a = (0..m).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    a.sort_by(|a1, a2| a1[1].cmp(&a2[1]));

    let mut east = 0;
    let mut ans = 0;
    for ab in a.iter() {
        if ab[0] >= east {
            east = ab[1];
            ans += 1;
        }
    }

    println!("{}", ans);
}