fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let c = read1::<String>().chars().collect::<Vec<char>>()[0];
    if c == 'a' || c == 'i' || c == 'u' || c == 'e' || c == 'o' {
        println!("vowel");
    } else {
        println!("consonant");
    }
}