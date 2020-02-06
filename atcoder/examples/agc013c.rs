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
    let nlt = readn::<usize>(" ");
    let n = nlt[0];
    let l = nlt[1];
    let t = nlt[2];
    let mut x = Vec::with_capacity(n);
    let mut w = Vec::with_capacity(n);
    let mut pos = Vec::with_capacity(n);

    for _ in 0..n {
        let xw = readn::<usize>(" ");
        x.push(xw[0]);
        w.push(xw[1]);

        if xw[1] == 1 {
            pos.push((xw[0] + t) % l);
        } else {
            if xw[0] % l < t % l {
                pos.push(l + xw[0] % l - t % l);
            } else {
                pos.push(xw[0] % l - t % l);
            }
        }
    }

    pos.sort();

    let mut count = 0;
    for i in 0..n {
        if w[i] == 1 {
            let cic = (x[i] + t) / l;
            if cic > count {
                count += n - cic;
            } else {
                count -= cic;
            }
        } else {
            count += (t + l - x[i] - 1) / l;
        }
    }

    let zero = (count % n) as usize;
    for i in 0..n {
        if i < zero {
            println!("{}", pos[n + i - zero]);
        } else {
            println!("{}", pos[i-(count%n)]);
        }
    }
}