pub fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

const INF: isize = 1_000_000_000_000;

fn main() {
    let nm = readn::<usize>(" ");
    let n = nm[0];
    let m = nm[1];
    let z = n * 2;

    let p = readn::<isize>(" ");
    let q = readn::<isize>(" ");

    let mut edge = vec![Vec::new(); z+1];

    for i in 0..n {
        edge[i].push((z, 0));
        edge[z].push((i, p[i]));
        edge[i+n].push((z, q[i]));
        edge[z].push((i+n, 0));
    }

    for _ in 0..m {
        let xyab = readn::<usize>(" ");
        let x = xyab[0] - 1;
        let y = xyab[1] - 1;
        let a = xyab[2] as isize;
        let b = xyab[3] as isize;
        edge[x].push((y+n, -a));
        edge[y+n].push((x, b));
    }

    let mut d = vec![INF; z+1];
    d[z] = 0;

    let mut update = true;
    while update {
        update = false;
        for i in 0..z+1 {
            for e in edge[i].iter() {
                let j = e.0;
                let dd = e.1;
                if d[j] > d[i] + dd {
                    update = true;
                    d[j] = d[i] + dd;
                }
            }
        }
        if d[z] < 0 {
            println!("no");
            return;
        }
    }

    println!("yes");
}