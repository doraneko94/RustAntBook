use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}


pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn rounds(x: usize) -> (usize, usize) {
    let h = x / 100;
    let mt = x % 10;
    let mh = x % 100 - mt;
    if mt >= 5 {
        (h, mh + 5)
    } else {
        (h, mh)
    }
}

fn rounde(x: usize) -> (usize, usize) {
    let mut h = x / 100;
    let mut mt = x % 10;
    let mut mh = (x % 100) / 10;
    if mt > 5 {
        mh += 1;
        mt = 0;
    } else if mt > 0 {
        mt = 5;
    } else {
        mt = 0;
    }
    if mh == 6 {
        mh = 0;
        h += 1;
    }
    (h, mh*10+mt)
}

fn tostring(x: usize) -> String {
    let s = x.to_string();
    if s.len() == 4 {
        return s;
    } else if s.len() == 3 {
        return "0".to_string() + &s;
    } else if s.len() == 2 {
        return "00".to_string() + &s;
    } else {
        return "000".to_string() + &s;
    }
}

fn main() {
    let n = read1::<usize>();
    let mut que = BinaryHeap::new();

    for _ in 0..n {
        let se = readn::<usize>("-");
        let s = se[0];
        let e = se[1];
        let (sh, sm) = rounds(s);
        let (eh, em) = rounde(e);
        que.push((Rev(sh*100+sm), Rev(eh*100+em)));
    }

    let (Rev(b0), Rev(e0)) = que.pop().unwrap();
    let mut ans = tostring(b0) + "-";
    let mut e = e0;
    while que.len() > 0 {
        let (Rev(b1), Rev(e1)) = que.pop().unwrap();
        if b1 > e {
            ans = ans + &tostring(e) + "\n" + &tostring(b1) + "-";
            e = e1;
        } else {
            e = std::cmp::max(e, e1);
        }
    }
    ans = ans + &tostring(e);

    println!("{}", ans);
}