fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn readn<T: std::str::FromStr>(delimiter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimiter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn filter(s: &mut Vec<Vec<char>>, word: &Vec<char>) {
    let l = s.len();
    let w = word.len();

    for i in 0..l {
        if s[i].len() == w {
            let mut flg = true;
            for j in 0..w {
                if word[j] != '*' && s[i][j] != word[j] {
                    flg = false;
                    break;
                }
            }
            if flg {
                s[i] = vec!['*'; w];
            }
        }
    }
}

fn main() {
    let mut s = readn::<String>(" ").iter().map(|v| v.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let n = read1::<usize>();

    for i in 0..n {
        let t = read1::<String>().chars().collect::<Vec<char>>();
        filter(&mut s, &t);
    }

    let ans = s.iter().map(|v| v.iter().map(|&c| c).collect::<String>() + " ").collect::<String>();
    println!("{}", ans);
}