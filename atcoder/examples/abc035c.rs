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
    let nq = readn::<usize>(" ");
    let n = nq[0];
    let q = nq[1];
    let mut board = vec![0; n+1];

    for _ in 0..q {
        let lr = readn::<usize>(" ");
        let l = lr[0] - 1;
        let r = lr[1] - 1;
        board[l] += 1;
        board[r + 1] += -1;
    }

    let mut ans = "".to_string();
    let mut t = 0;
    for i in 0..n {
        t += board[i];
        if t % 2 == 0 {
            ans += "0";
        } else {
            ans += "1";
        }
    }

    println!("{}", ans);
}