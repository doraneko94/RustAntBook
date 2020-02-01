use library::io::read1;
use std::collections::VecDeque;

const INF: usize = 100000000;

fn bfs(n: usize, m: usize, field: &[Vec<char>]) -> usize {
    let mut queue = VecDeque::new();
    let mut d = (0..n).map(|_| (0..m).map(|_| INF).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();
    let dydx = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut sy = INF;
    let mut sx = INF;
    let mut gy = INF;
    let mut gx = INF;

    for y in 0..n {
        for x in 0..m {
            if field[y][x] == 'S' {
                sy = y;
                sx = x;
            }
            if field[y][x] == 'G' {
                gy = y;
                gx = x;
            }
        }
    }

    if sy == INF || sx == INF || gy == INF || gx == INF { return INF; }

    queue.push_back((sy, sx));
    d[sy][sx] = 0;

    while queue.len() > 0 {
        let p = queue.pop_front().unwrap();
        if p.0 == gy && p.1 == gx { break; }

        for (dy, dx) in dydx.iter() {
            let ny: i32 = p.0 as i32 + dy;
            let nx: i32 = p.1 as i32 + dx;
            if ny >= 0 && ny < n as i32 && nx >= 0 && nx < m as i32 && field[ny as usize][nx as usize] != '#' && d[ny as usize][nx as usize] >= INF {
                queue.push_back((ny as usize, nx as usize));
                d[ny as usize][nx as usize] = d[p.0][p.1] + 1;
            }
        }
    }

    d[gy][gx]
}

fn main() {
    let n = read1::<usize>();
    let m = read1::<usize>();
    let field = (0..n).map(|_| read1::<String>().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    println!("{}", bfs(n, m, &field));
}