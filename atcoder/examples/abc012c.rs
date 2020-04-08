pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();

    let mut sum = 0;
    for i in 1..10 {
        for j in 1..10 {
            sum += i * j;
        }
    }

    let mut ans = "".to_string();
    for i in 1..10 {
        for j in 1..10 {
            if sum - i * j == n {
                ans = ans + &(i.to_string()) + " x " + &(j.to_string()) + "\n";
            }
        }
    }

    print!("{}", ans);
}