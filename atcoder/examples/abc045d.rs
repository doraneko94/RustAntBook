use std::collections::HashMap;

fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let hwn = readn::<usize>(" ");
    let h = hwn[0];
    let w = hwn[1];
    let n = hwn[2];
    
    let mut dict = HashMap::new();

    for _ in 0..n {
        let ab = readn::<usize>(" ");
        let a = ab[0] - 1;
        let b = ab[1] - 1;
        for j in 0..3 {
            if a + j < 2 || a + j >= h { continue; }
            let y = a + j - 2;
            for k in 0..3 {
                if b + k < 2 || b + k >= w { continue; }
                let x = b + k - 2;
                let count = dict.entry((y, x)).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut cnt = vec![0usize; 10];

    for (_, &v) in dict.iter() {
        cnt[v] += 1;
    }

    let zero = (h - 2) * (w - 2) - cnt.iter().sum::<usize>();
    let mut ans = zero.to_string() + "\n";
    for i in 1..10 {
        ans = ans + &(cnt[i].to_string()) + "\n";
    }

    print!("{}", ans);
}