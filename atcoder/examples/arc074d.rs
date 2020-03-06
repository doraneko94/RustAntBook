use std::collections::BinaryHeap;

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
    let mut que = BinaryHeap::with_capacity(n * 3);

    for i in 0..(n*3) {
        let a = read1::<usize>();
        que.push((a, i));
    }

    //let mut count = 0;
    //let mut head = 0;
    //let mut tail = 0;
}