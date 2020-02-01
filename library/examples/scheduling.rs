use library::io::{read1, readn};

fn main() {
    let _n = read1::<usize>();
    let s = readn::<usize>(" ");
    let t = readn::<usize>(" ");

    let mut ts = t.iter().zip(s.iter()).map(|(&a, &b)| (a, b)).collect::<Vec<(usize, usize)>>();
    ts.sort_by(|a, b| (a.0).cmp(&b.0));
    
    let mut time = 0;
    let mut ans = 0;
    for &work in ts.iter() {
        if work.1 > time {
            time = work.0;
            ans += 1;
        } 
    }

    println!("{:?}", ans);
}