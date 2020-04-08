fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read1::<usize>();
    for i in 0..50_000 {
        if (i as f64 * 1.08) as usize == n {
            println!("{}", i);
            return;
        } 
    }
    println!(":(");
}