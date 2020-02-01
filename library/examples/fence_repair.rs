use std::collections::VecDeque;
use library::io::{read1, readn};

fn priority_push(v: &mut VecDeque<usize>, e: usize) {
    for i in 0..v.len() {
        if v[i] >= e {
            v.insert(i, e);
            return;
        }
    }
    v.push_back(e);
}

fn main() {
    let _n = read1::<usize>();
    let mut l = readn::<usize>(" ");
    l.sort();
    let mut l = VecDeque::from(l);

    let mut ans = 0;

    while l.len() > 1 {
        let p1 = l.pop_front().unwrap();
        let p2 = l.pop_front().unwrap();
        ans += p1 + p2;
        priority_push(&mut l, p1 + p2);
    }

    println!("{}", ans);
}