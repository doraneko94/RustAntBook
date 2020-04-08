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
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];
    let d = readn::<usize>(" ");
    let mut num = vec![0; 10];
    for i in 0..k {
        num[d[i]] = 1;
    }

    for i in n..1_000_000 {
        let s = i.to_string().chars().collect::<Vec<char>>();
        let mut flg = true;
        for i in 0..s.len() {
            if num[s[i] as usize - 48] == 1 {
                flg = false;
                break;
            }
        }
        if flg {
            println!("{}", i);
            return;
        }
    }
}