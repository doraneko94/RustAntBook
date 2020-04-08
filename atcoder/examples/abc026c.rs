pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn rec(i: usize, v: &Vec<Vec<usize>>) -> usize {
    if v[i].len() == 0 {
        return 1;
    }

    let mut v_max = 0;
    let mut v_min = 1_000_000_000;
    for &j in v[i].iter() {
        let s = rec(j, v);
        v_max = std::cmp::max(v_max, s);
        v_min = std::cmp::min(v_min, s);
    }

    v_max + v_min + 1
}

fn main() {
    let n = read1::<usize>();
    let mut v = vec![Vec::new(); n];

    for i in 1..n {
        let b = read1::<usize>() - 1;
        v[b].push(i);
    }

    println!("{}", rec(0, &v));
}