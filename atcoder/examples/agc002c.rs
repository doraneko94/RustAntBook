fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let nl = readn::<usize>(" ");
    let n = nl[0];
    let l = nl[1];

    let a = readn::<usize>(" ");

    let mut last = 0;
    for i in 1..n {
        if a[i-1] + a[i] >= l {
            last = i;
            break;
        }
    }
    if last == 0 {
        println!("Impossible");
    } else {
        println!("Possible");
        let mut ans = "".to_string();
        for i in 1..last {
            ans = ans + &(i.to_string()) + "\n";
        }
        for i in (last+1..n).rev() {
            ans = ans + &(i.to_string()) + "\n"
        }
        ans = ans + &(last.to_string());
        println!("{}", ans);
    }
}