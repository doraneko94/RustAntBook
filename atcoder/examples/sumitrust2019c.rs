fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let x = read1::<usize>();
    let time = x / 100;
    let rest = x % 100;
    if time >= 20 {
        println!("1");
    } else {
        for a in 0..time+1 {
            for b in 0..(time+1-a) {
                for c in 0..(time+1-a-b) {
                    for d in 0..(time+1-a-b-c) {
                        for e in 0..(time+1-a-b-c-d) {
                            for f in 0..(time+1-a-b-c-d-e) {
                                if b + 2*c + 3*d + 4*e + 5*f == rest {
                                    println!("1");
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("0");
    }
}