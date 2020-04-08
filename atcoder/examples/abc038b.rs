fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let hw1 = readn::<usize>(" ");
    let hw2 = readn::<usize>(" ");
    if hw1[0] == hw2[0] || hw1[0] == hw2[1] || hw1[1] == hw2[0] || hw1[1] == hw2[1] {
        println!("YES");
    } else {
        println!("NO");
    }
}