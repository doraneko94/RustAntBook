fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>().chars().collect::<Vec<char>>();
    let t = read1::<String>().chars().collect::<Vec<char>>();

    let mut flg = true;
    for i in 0..s.len() {
        if s[i] != t[i]  {
            if s[i] == '@' && (t[i] == 'a' || t[i] == 't' || t[i] == 'c' || t[i] == 'o' || t[i] == 'd' || t[i] == 'e' || t[i] == 'r') {
                continue;
            } else if t[i] == '@' && (s[i] == 'a' || s[i] == 't' || s[i] == 'c' || s[i] == 'o' || s[i] == 'd' || s[i] == 'e' || s[i] == 'r') {
                continue;
            } else {
                flg = false;
                break;
            }
        }
    }

    if flg {
        println!("You can win");
    } else {
        println!("You will lose");
    }
}