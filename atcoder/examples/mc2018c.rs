pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn dfs(i: usize, a: usize, edge: &Vec<usize>, angel: &mut Vec<usize>) -> bool {
    let j = edge[i];
    angel[i] = a ^ 1;
    if angel[j] == 2 {
        let f = dfs(j, a ^ 1, edge, angel);
        if !f {
            return false;
        }
    } else {
        if angel[j] != a { return false; }
    }
    true
}

fn main() {
    let n = read1::<usize>();
    let mut angel = vec![2; n];
    let mut edge = vec![0; n];

    for i in 0..n {
        let a = read1::<usize>() - 1;
        edge[i] = a;
    }

    for i in 0..n {
        if angel[i] == 2 {
            let flg = dfs(i, 0, &edge, &mut angel);
            if !flg {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", n / 2);
}