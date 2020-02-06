pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut a = Vec::with_capacity(10);
    let mut all = 0;
    for _ in 0..10 {
        let tmp = read1::<String>().chars().collect::<Vec<char>>();
        all += tmp.iter().map(|&c| {
            if c == 'o' { 1 }
            else { 0 }
        }).sum::<usize>();
        a.push(tmp);
    }
    
    for y in 0..10 {
        for x in 0..10 {
            if a[y][x] == 'x' {
                let mut ac = a.clone();
                if dfs(y, x, &mut ac) == all + 1 {
                    println!("YES");
                    return;
                }
            }
        }
    }

    println!("NO");
}

fn dfs(y: usize, x: usize, a: &mut [Vec<char>]) -> usize {
    a[y][x] = 'x';
    let mut s = 1;

    if y + 1 < 10 && a[y + 1][x] == 'o' {
        s += dfs(y + 1, x, a);
    }

    if y >= 1 && a[y - 1][x] == 'o' {
        s += dfs(y - 1, x, a);
    }

    if x + 1 < 10 && a[y][x + 1] == 'o' {
        s += dfs(y, x + 1, a);
    }

    if x >= 1 && a[y][x - 1] == 'o' {
        s += dfs(y, x - 1, a);
    }

    s
}