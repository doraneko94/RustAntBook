use std::collections::HashSet;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let k = read1::<usize>();

    let mut set = HashSet::new();
    let l = s.len();

    for i in 0..l {
        if i + k <= l {
            set.insert((i..i+k).map(|j| s[j]).collect::<String>());
        }
    }

    println!("{}", set.len());
}