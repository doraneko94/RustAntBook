fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn dfs(n: usize, m: u8, v: Vec<u8>) -> String {
    let l = v.len();
    if l == n {
        return v.iter().map(|&i| i as char).collect::<String>() + "\n";
    }

    let mut ans = "".to_string();
    for i in 97..m+1 {
        let mut v2 = v.clone();
        v2.push(i);
        ans = ans + &dfs(n, m, v2);
    }
    let mut v2 = v.clone();
    v2.push(m+1);
    ans = ans + &dfs(n, m+1, v2);

    return ans;
}

fn main() {
    let n = read1::<usize>();

    println!("{}", dfs(n, 97u8, vec![97u8]));
}