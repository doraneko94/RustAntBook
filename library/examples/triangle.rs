use library::io::{read1, readn};

fn main() {
    let n = read1::<usize>();
    let a = readn::<usize>(" ");

    let mut ans = 0;
    for i in 0..n-2 {
        let a1 = a[i];
        for j in i+1..n-1 {
            let a2 = a[j];
            for k in j+1..n {
                let a3 = a[k];
                if (a1 < a2 + a3) && (a2 < a3 + a1) && (a3 < a1 + a2) {
                    let perimeter = a1 + a2 + a3;
                    if ans < perimeter { ans = perimeter; }
                }
            }
        }
    }
    println!("{}", ans);
}