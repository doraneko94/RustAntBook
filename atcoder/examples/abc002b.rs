pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let w = read1::<String>().chars().collect::<Vec<char>>();
    let mut ans = Vec::new();

    for &c in w.iter() {
        if c != 'a' && c != 'i' && c != 'u' && c != 'e' && c != 'o' {
            ans.push(c);
        } 
    }

    println!("{}", ans.iter().map(|&c| c).collect::<String>());
}