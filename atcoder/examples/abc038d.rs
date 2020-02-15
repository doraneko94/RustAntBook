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
    let n = read1::<usize>();
    let mut wh = (0..n).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    wh.sort_by(|a, b| {
        if a[1] == b[1] {
            b[0].cmp(&a[0])
        } else {
            a[1].cmp(&b[1])
        }
    });
    let mut lis = Vec::new();
    let mut ans = 0;
    for v in wh.iter() {
        if ans == 0 {
            ans += 1;
            lis.push(v[0]);
        } else {
            if v[0] > lis[ans - 1] {
                lis.push(v[0]);
                ans += 1;
            } else {
                match lis.binary_search(&v[0]) {
                    Ok(index) => lis[index] = v[0],
                    Err(index) => lis[index] = v[0],
                };
                
            }
        }
    }

    println!("{}", ans);
}