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
    let n = read1::<usize>();
    let mut pos = (0..n).map(|_| {
        let xl = readn::<isize>(" ");
        (xl[0] - xl[1], xl[0] + xl[1])
    }).collect::<Vec<(isize, isize)>>();
    pos.sort_by(|a, b| (a.1).cmp(&b.1));

    let mut right = pos[0].0;
    let mut ans = 0;
    for p in pos.iter() {
        if p.0 >= right {
            right = p.1;
            ans += 1;
        }
    }

    println!("{}", ans);
}