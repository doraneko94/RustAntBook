pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: isize = 100000;

fn main() {
    let dn = readn::<usize>(" ");
    let d = dn[0];
    let n = dn[1];

    let temp = (0..d).map(|_| read1::<isize>()).collect::<Vec<isize>>();
    let mut abc = (0..n).map(|_| readn::<isize>(" ")).collect::<Vec<Vec<isize>>>();
    abc.sort_by(|a, b| (a[0]).cmp(&b[0]));
    let mut dp = vec![vec![-1; 2]; d];
    let mut save = vec![-1, -1];
    for i in 0..d {
        let mut ls = vec![-1, INF];
        let mut index = 0;
        while index < n && abc[index][0] <= temp[i] {
            if abc[index][1] >= temp[i] {
                if ls[1] > abc[index][2] {
                    ls[1] = abc[index][2];
                }
                if ls[0] < abc[index][2] {
                    ls[0] = abc[index][2];
                }
            }
            index += 1;
        }
        if i == 0 {
            dp[i][0] = 0;
            dp[i][1] = 0;
        } else {
            for j in 0..2 {
                for k in 0..2 {
                    dp[i][j] = std::cmp::max(dp[i][j], dp[i-1][k] + (save[k] - ls[j]).abs());
                }
            }
        }
        save[0] = ls[0];
        save[1] = ls[1]; 
    }
    println!("{}", std::cmp::max(dp[d-1][0], dp[d-1][1]));
}