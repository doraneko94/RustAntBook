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
    let dg = readn::<usize>(" ");
    let d = dg[0];
    let g = dg[1];
    let mut p = Vec::with_capacity(d);
    let mut c = Vec::with_capacity(d);

    for _ in 0..d {
        let pc = readn::<usize>(" ");
        p.push(pc[0]);
        c.push(pc[1]);
    }

    let mut ans = p.iter().sum::<usize>();
    for i in 0..2_usize.pow(d as u32) {
        let mut score = 0;
        let mut half = d;
        let mut num = 0;
        for j in 0..d {
            let bit = i >> j & 1;
            if bit == 1 {
                score += p[j] * 100 * (j + 1) + c[j];
                num += p[j];
            } else {
                half = j;
            }
        }
        if half < 10 {
            if score < g {
                let half_num = ((g - score) + 100 * (half + 1) - 1) / (100 * (half + 1));
                score += 100 * (half + 1) * std::cmp::min(half_num, p[half]);
                num += half_num;
            }
        }
        if score >= g && num < ans {
            ans = num;
        }
    }

    println!("{}", ans);
}