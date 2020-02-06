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
    let s = read1::<String>().chars().map(|c| c as isize - 48).collect::<Vec<isize>>();
    
    for i in 0..2_usize.pow(3) {
        let mut ans = s[0].to_string();
        let mut tmp = s[0];
        for j in 0..3 {
            if i >> j & 1 == 1 {
                ans += "+";
                tmp += s[j + 1];
            } else {
                ans += "-";
                tmp -= s[j + 1];
            }
            ans += &s[j + 1].to_string();
        }

        if tmp == 7 {
            println!("{}=7", ans);
            return;
        }
    }
}