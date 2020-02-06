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
    let n = read1::<usize>();
    let k = read1::<usize>();
    let c = (0..n).map(|_| read1::<String>()).collect::<Vec<String>>();
    let indices = (0..n).collect::<Vec<usize>>();
    let mut ans = Vec::new();
    let perm = permutation(&indices, k, false);

    for v in perm.iter() {
        let mut a = c[v[0]].clone();
        for i in 1..k {
            a += &c[v[i]];
        }
        let au = a.parse::<usize>().ok().unwrap();
        match ans.binary_search(&au) {
            Ok(_) => continue,
            Err(pos) => ans.insert(pos, au),
        }
    }

    println!("{}", ans.len());
}

fn permutation<T: Clone + Copy>(v: &Vec<T>, m: usize, duplicate: bool) -> Vec<Vec<T>> {
    let n = v.len();
    let mut incl = m - 1;
    let mut v_tmp = vec![0; m];
    let mut ret = Vec::new();
    if duplicate {
        ret.push(vec![v[0]; m]);
    }
    while incl > 0 || v_tmp[0] < n - 1 {
        if v_tmp[incl] == n - 1 {
            v_tmp[incl] = 0;
            incl -= 1;
            continue;
        }
        v_tmp[incl] += 1;
        incl = m - 1;
        let mut flag = true;
        if !duplicate {
            let mut test = vec![0; n];
            for &i in v_tmp.iter() {
                if test[i] == 1 {
                    flag = false;
                    break;
                } else {
                    test[i] = 1;
                }
            }
        }
        if flag {
            ret.push(v_tmp.iter().map(|&index| v[index]).collect::<Vec<T>>());
        }
    }
    ret
}