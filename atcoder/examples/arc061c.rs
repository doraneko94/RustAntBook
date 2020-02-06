pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let s = read1::<String>().chars().map(|c| c as usize - 48).collect::<Vec<usize>>();
    let mut ans = 0;
    for i in 0..2_usize.pow(s.len() as u32 - 1) {
        let mut tmp = vec![s[0]];
        let mut count = 0;
        for j in 1..s.len() {
            if i>>(j-1)&1 == 1 {
                tmp[count] *= 10;
                tmp[count] += s[j];
            } else {
                tmp.push(s[j]);
                count += 1;
            }
        }
        ans += tmp.iter().sum::<usize>();
    }
    println!("{}", ans);
}