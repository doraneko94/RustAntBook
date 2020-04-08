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
    let n = read1::<usize>();
    let mut v = vec![vec![0; 26]; n];

    for i in 0..n {
        let s = read1::<String>().chars().collect::<Vec<char>>();
        for c in s.iter() {
            v[i][*c as usize - 97] += 1;
        }
    }

    let mut ans = "".to_string();
    for i in 0..26 {
        let mut k = 100;
        for j in 0..n {
            k = std::cmp::min(k, v[j][i]);
        }
        for _ in 0..k {
        	ans = ans + &(((i as u8 + 97) as char).to_string());
        }
    }

    println!("{}", ans);
}