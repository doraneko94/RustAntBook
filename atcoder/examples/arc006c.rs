pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    let mut wlist = Vec::new();
    for _ in 0..n {
        let w = read1::<usize>();
        let mut back_flg = true;
        let wsize = wlist.len();
        for i in 0..wsize {
            if wlist[i] >= w {
                wlist[i] = w;
                back_flg = false;
                break;
            }
        }
        if back_flg {
            wlist.push(w);
        }
    }
    println!("{}", wlist.len());
}