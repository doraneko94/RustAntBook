pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: isize = 1_000_000_000_000_000;

fn main() {
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];
    let edge = (0..m).map(|_| readn::<isize>(" ")).collect::<Vec<Vec<isize>>>();
    let mut d = vec![0; n];

    let mut count = 0;
    let mut n_update = false;
    while count <= n {
        let mut flg = true;
        for e in edge.iter() {
            let a = (e[0] - 1) as usize;
            let b = (e[1] - 1) as usize;
            let c = e[2];
            if d[b] < d[a] + c {
                flg = false;
                d[b] = d[a] + c;
                if b == n - 1 && count == n {
                    n_update = true;
                }
            }
        }
        if flg { break; }
        count += 1;
    }
    if !n_update {
        let mut d = vec![INF; n];
        let mut count = 0;
        d[0] = 0;
        while count < n {
            let mut flg = true;
            for e in edge.iter() {
                let a = (e[0] - 1) as usize;
                let b = (e[1] - 1) as usize;
                let c = e[2];
                if d[b] > d[a] - c {
                    flg = false;
                    d[b] = d[a] - c;
                }
            }
            if flg { break; }
            count += 1;
        }
        println!("{}", d[n-1] * -1);
    } else {
        println!("inf");
    }
}