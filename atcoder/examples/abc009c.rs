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
    let nk = readn::<usize>(" ");
    let n = nk[0];
    let k = nk[1];
    let s = read1::<String>().chars().collect::<Vec<char>>();

    let mut cnt = vec![vec![0; 26]; n+1];
    
    for i in 1..n+1 {
        for j in 0..26 {
            if s[i-1] == std::char::from_u32(j as u32 + 97).unwrap() {
                cnt[i][j] = cnt[i-1][j] + 1;
            } else {
                cnt[i][j] = cnt[i-1][j];
            }
        }
    }

    let mut t = Vec::new();
    let mut tcnt = vec![0; 26];
    let mut diff = 0;
    let mut s_sort = s.clone();
    s_sort.sort();

    for i in 1..n+1 {
        let mut c = Vec::with_capacity(26);
        let c_sum = tcnt.iter().zip(cnt[i].iter()).map(|(&tc, &cn)| {
            if tc < cn {
                c.push(cn - tc);
                cn - tc
            } else {
                c.push(0);
                0
            }
        }).sum::<usize>();
        let s_sort_len = s_sort.len();
        for j in 0..s_sort_len {
            let mut diff1 = diff;
            let ssj = s_sort[j];
            if ssj != s[i-1] {
                diff1 += 1;
            }
            let mut diff2 = c_sum;
            if c[ssj as usize - 97] > 0 {
                diff2 -= 1;
            }
            if diff1 + diff2 <= k {
                t.push(ssj);
                s_sort.remove(j);
                diff = diff1;
                tcnt[ssj as usize - 97] += 1;
                break;
            }
        }
    }
    println!("{}", t.into_iter().collect::<String>());
}