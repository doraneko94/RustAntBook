use std::collections::HashMap;

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
    let rck = readn::<usize>(" ");
    let r = rck[0];
    let c = rck[1];
    let k = rck[2];

    let n = read1::<usize>();
    let mut rv = vec![0; r];
    let mut cv = vec![0; c];
    let mut candy = Vec::with_capacity(n);

    for _ in 0..n {
        let rc = readn::<usize>(" ");
        rv[rc[0]-1] += 1;
        cv[rc[1]-1] += 1;
        candy.push(rc);
    }

    let mut rd = HashMap::new();
    let mut cd = HashMap::new();
    for i in 0..r {
        let count = rd.entry(rv[i]).or_insert(0usize);
        *count += 1;
    }
    for i in 0..c {
        let count = cd.entry(cv[i]).or_insert(0usize);
        *count += 1;
    }
    
    let mut ans = 0usize;
    for (&key, &val) in rd.iter() {
        if k >= key {
            match cd.get(&(k - key)) {
                Some(v) => {
                    ans += v * val;
                }
                _ => continue,
            };
        }
    }

    for rc in candy.iter() {
        if rv[rc[0]-1] + cv[rc[1]-1] == k {
            ans -= 1;
        }
        if rv[rc[0]-1] + cv[rc[1]-1] == k + 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}