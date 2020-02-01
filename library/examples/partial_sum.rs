use library::io::{read1, readn};

fn dfs(i: usize, sum: i64, n: usize, k: i64, v: &[i64]) -> bool {
    if i == n { return sum == k; }

    if dfs(i + 1, sum, n, k, v) { return true; }

    if dfs(i + 1, sum + v[i], n, k, v) { return true; }

    false
}

fn main() {
    let n = read1::<usize>();
    let a = readn::<i64>(" ");
    let k = read1::<i64>();

    if dfs(0, 0, n, k, &a) { println!("Yes"); }
    else { println!("No"); }
}