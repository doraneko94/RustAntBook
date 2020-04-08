pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let t = read1::<usize>();
    let mut v = vec![0; 4];
    let mut q = 0;

    for &c in s.iter() {
        if c == 'L' {
            v[0] += 1;
        } else if c == 'R' {
            v[1] += 1;
        } else if c == 'U' {
            v[2] += 1;
        } else if c == 'D' {
            v[3] += 1;
        } else {
            q += 1;
        }
    }

    let lr = if v[0] > v[1] {
        v[0] - v[1]
    } else {
        v[1] - v[0]
    };
    let ud = if v[2] > v[3] {
        v[2] - v[3]
    } else {
        v[3] - v[2]
    };

    if t == 1 {
        println!("{}", lr + ud + q);
    } else {
        if lr + ud >= q {
            println!("{}", lr + ud - q);
        } else {
            let rest = q - lr - ud;
            println!("{}", rest % 2);
        }
    }
}