use library::io::{read1, readn};

fn main() {
    let l = read1::<usize>();
    let _n = read1::<usize>();
    let x = readn::<usize>(" ");

    let mut min = 0;
    let mut max = 0;
    for &pos in x.iter() {
        if pos < l - pos {
            if min < pos { min = pos; }
            if max < l - pos { max = l - pos; }
        } else {
            if min < l - pos { min = l - pos; }
            if max < pos { max = pos; }
        }
    }

    println!("{} {}", min, max);
}