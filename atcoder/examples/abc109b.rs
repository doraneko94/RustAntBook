use std::collections::HashSet;

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
    let mut dict = HashSet::new();
    let mut flg = true;
    let mut back = '*';

    for _ in 0..n {
        let w = read1::<String>();
        let ws = (&w).to_string();

        if dict.contains(&w) {
            flg = false;
            break;
        } else {
            dict.insert(ws);
            let c = w.chars().collect::<Vec<char>>();
            if back != '*' && c[0] != back {
                flg = false;
                break;
            } else {
                back = c[c.len()-1];
            }
        }
    }

    if flg {
        println!("Yes");
    } else {
        println!("No");
    }
}