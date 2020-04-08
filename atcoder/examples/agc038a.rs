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
    let hwab = readn::<usize>(" ");
    let h = hwab[0];
    let w = hwab[1];
    let a = hwab[2];
    let b = hwab[3];

    let mut s = vec![vec!['0'; w]; h];

    for i in 0..h {
        for j in 0..w {
            if i >= b && j < a {
                s[i][j] = '1';
            } else if i < b && j >= a {
                s[i][j] = '1';
            }
        }
    }

    let mut ans = "".to_string();
    for i in 0..h {
        ans = ans + &(s[i].iter().map(|&c| c).collect::<String>()) + "\n";
    }
    print!("{}", ans);
}