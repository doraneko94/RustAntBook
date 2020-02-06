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
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];
    let mut v = vec![vec![0; n]; n];
    
    for _ in 0..m {
        let xy = readn::<usize>(" ");
        v[xy[0] - 1][xy[1] - 1] = 1;
    }

    let mut ans = 0;
    for i in 0..2_usize.pow(n as u32) {
        let mut member = Vec::new();
        for j in 0..n {
            if i >> j & 1 == 1 {
                member.push(j as usize);
            }
        }
        let mut flag = true;
        for &x in member.iter() {
            for &y in member.iter() {
                if x < y {
                    if v[x][y] == 0 {
                        flag = false;
                        break;
                    }
                }
            }
        }
        if flag {
            ans = std::cmp::max(ans, member.len());
        }
    }
    println!("{}", ans);
}