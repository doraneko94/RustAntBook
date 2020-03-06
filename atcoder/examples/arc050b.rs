fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: usize = 1_000_000_000_000_000_000;

fn judge(r: usize, b: usize, x: usize, y: usize, mid: usize) -> bool {
    if r < mid || b < mid {
        return false; 
    } else if (r - mid) / (x - 1) + (b - mid) / (y - 1) >= mid {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let rb = readn::<usize>(" ");
    let r = rb[0];
    let b = rb[1];
    let xy = readn::<usize>(" ");
    let x = xy[0];
    let y = xy[1];

    let mut low = 0;
    let mut high = INF;

    while low != high {
        let mid = (low + high) / 2;
        if judge(r, b, x, y, mid) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    println!("{}", low - 1);
}