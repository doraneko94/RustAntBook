fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn inspect(a: &Vec<char>) -> (bool, usize, usize) {
    let mut l = 0;
    let mut r = 0;
    let mut flg = false;
    let mut cont = false;
    let w = a.len();
    for i in 0..w {
        if a[i] == '#' && !cont {
            if flg { return (false, 0, 0); }
            cont = true;
            l = i;
        }
        if a[i] == '.' && cont {
            cont = false;
            r = i - 1;
            flg = true;
        }   
    }
    if r == 0 && cont {
        r = w - 1;
        flg = true;
    }

    (flg, l, r)
}

fn main() {
    let hw = readn::<usize>(" ");
    let h = hw[0];
    let _w = hw[1];

    let mut rr = 0;
    for _ in 0..h {
        let a = read1::<String>().chars().collect::<Vec<char>>();
        let (f, l, r) = inspect(&a);
        if !f || rr != l {
            println!("Impossible");
            return;
        } else {
            rr = r;
        }
    }

    println!("Possible");
}