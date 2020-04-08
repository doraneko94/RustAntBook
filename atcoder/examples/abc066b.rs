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
    let mut s = read1::<String>().chars().collect::<Vec<char>>();
    s.pop();
    s.pop();
    loop {
        let h = s.len() / 2;
        let mut flg = true;
        for i in 0..h {
            if s[i] != s[i+h] {
                flg = false;
                break;
            }
        }
        if flg {
            break;
        } else {
            s.pop();
            s.pop();
        }
    }
    
    
    println!("{}", s.len());
}