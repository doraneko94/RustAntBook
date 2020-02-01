use library::io::{read1, readn};

fn main() {
    let mut c = readn::<usize>(" ");
    c.reverse();
    if c.len() != 6 { println!("No"); }
    let mut a = read1::<usize>();

    let mut ans = 0;
    let v = vec![500, 100, 50, 10, 5, 1];

    for i in 0..=5 {
        let num = std::cmp::min(c[i], a / v[i]);
        ans += num;
        a -= v[i] * num;
    }

    println!("{}", ans);
}