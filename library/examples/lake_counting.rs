use library::io::read1;

fn main() {
    let n = read1::<usize>();
    let m = read1::<usize>();
    let mut field = Vec::with_capacity(n);
    for _ in 0..n { field.push(read1::<String>().chars().collect::<Vec<char>>()); }
    
    let mut ans: usize = 0;

    for y in 0..n {
        for x in 0..m {
            if field[y][x] == 'W' {
                dfs(y, x, n, m, &mut field);
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn dfs(y: usize, x: usize, n: usize, m: usize, field: &mut [Vec<char>]) {
    field[y][x] = '.';
    for dy in -1..=1 {
        for dx in -1..=1 {
            let next_y = y as i32 + dy;
            let next_x = x as i32 + dx;
            if next_y >= 0 && next_y < n as i32 && next_x >= 0 && next_x < m as i32 && field[next_y as usize][next_x as usize] == 'W' {
                dfs(next_y as usize, next_x as usize, n, m, field);
            }
        }
    }
}