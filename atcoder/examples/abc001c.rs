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
    let dd = readn::<usize>(" ");
    let deg = dd[0];
    let dis = dd[1] * 100;
    let tmp = dis / 60;
    let v = if tmp % 10 >= 5 {
        tmp / 10 + 1
    } else {
        tmp / 10
    };
    let pow = if v <= 2 {
        0
    } else if v <= 15 {
        1
    } else if v <= 33 {
        2
    } else if v <= 54 {
        3
    } else if v <= 79 {
        4
    } else if v <= 107 {
        5
    } else if v <= 138 {
        6
    } else if v <= 171 {
        7
    } else if v <= 207 {
        8
    } else if v <= 244 {
        9
    } else if v <= 284 {
        10
    } else if v <= 326 {
        11
    } else {
        12
    };
    let arg = if pow == 0 {
        "C"
    } else if deg <= 112 || deg >= 3488 {
        "N"
    } else if deg <= 337 {
        "NNE"
    } else if deg <= 562 {
        "NE"
    } else if deg <= 787 {
        "ENE"
    } else if deg <= 1012 {
        "E"
    } else if deg <= 1237 {
        "ESE"
    } else if deg <= 1462 {
        "SE"
    } else if deg <= 1687 {
        "SSE"
    } else if deg <= 1912 {
        "S"
    } else if deg <= 2137 {
        "SSW"
    } else if deg <= 2362 {
        "SW"
    } else if deg <= 2587 {
        "WSW"
    } else if deg <= 2812 {
        "W"
    } else if deg <= 3037 {
        "WNW"
    } else if deg <= 3262 {
        "NW"
    } else {
        "NNW"
    };

    println!("{} {}", arg, pow);
}