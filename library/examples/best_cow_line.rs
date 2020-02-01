use std::collections::VecDeque;
use library::io::read1;

fn main() {
    let n = read1::<usize>();
    let s = read1::<String>();

    let mut que = s.chars().collect::<VecDeque<char>>();
    let mut euq = s.chars().collect::<Vec<char>>();
    euq.reverse();
    let mut euq = VecDeque::from(euq);
    let mut ans = String::with_capacity(n);

    while que.len() > 0 {
        let mut left = false;
        let half = (que.len() + 1) / 2;
        for i in 0..half {
            if que[i] < euq[i] {
                left = true;
                break;
            }
            if que[i] > euq[i] {
                break;
            }
        }
        if left {
            ans.push(que.pop_front().unwrap());
            euq.pop_back();
        } else {
            ans.push(que.pop_back().unwrap());
            euq.pop_front();
        }
    }

    println!("{}", ans);
}