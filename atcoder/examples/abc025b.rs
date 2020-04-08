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
    let nab = readn::<isize>(" ");
    let n = nab[0];
    let a = nab[1];
    let b = nab[2];
    let mut pos = 0;

    for _ in 0..n {
        let sd = readn::<String>(" ");
        let s = &sd[0];
        let d: isize = sd[1].parse().ok().unwrap();

        let dist = if d < a {
            a
        } else if d > b {
            b
        } else {
            d
        };
        if s == "East" {
            pos += dist;
        } else {
            pos -= dist;
        }
    }

    if pos < 0 {
        println!("West {}", -pos);
    } else if pos > 0 {
        println!("East {}", pos);
    } else {
        println!("0");
    }
}