use library::io::{read1, readn};

fn main() {
    let n = read1::<usize>();
    let r = read1::<usize>();
    let mut x = readn::<usize>(" ");

    x.sort();
    let mut i = 0;
    let mut ans = 0;

    while i < n {
        let s = x[i];
        i += 1;

        while i < n && x[i] <= s + r {
            i += 1;
        }

        let p = x[i - 1];
        while i < n && x[i] <= p + r {
            i += 1;
        }

        ans += 1;
    }

    println!("{}", ans);
}