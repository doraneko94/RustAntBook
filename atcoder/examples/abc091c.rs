pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 10000;

fn main() {
    let n = read1::<usize>();
    let mut ab = (0..n).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    let mut cd = (0..n).map(|_| readn::<usize>(" ")).collect::<Vec<Vec<usize>>>();
    ab.sort_by(|a, b| b[0].cmp(&a[0]));
    cd.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut ans = 0;

    for i in 0..n {
        for j in 0..n {
            if ab[j][0] < cd[i][0] && ab[j][1] < cd[i][1] {
                ans += 1;
                ab[j] = vec![INF, INF];
                break;
            }
        }
    }

    println!("{}", ans);
}