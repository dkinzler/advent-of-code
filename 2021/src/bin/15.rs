use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let risk = read_input("inputs/15");
    // input is square
    let n = risk.len();
    let mut d = vec![vec![1 << 30; n]; n];
    d[0][0] = 0;
    let mut q = Vec::new();
    let mut qi = 0;
    q.push((0, 0));
    while qi < q.len() {
        let (r, c) = q[qi];
        qi += 1;
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr >= (n as i32) || nc < 0 || nc >= (n as i32) {
                continue;
            }
            let v = (risk[nr as usize][nc as usize] as i32) + d[r as usize][c as usize];
            if v < d[nr as usize][nc as usize] {
                d[nr as usize][nc as usize] = v;
                q.push((nr, nc));
            }
        }
    }
    println!("part1: {}", d[n - 1][n - 1]);
}

fn part2() {
    let risk = read_input("inputs/15");
    // input is square
    let n = risk.len();
    let mut d = vec![vec![1 << 30; 5 * n]; 5 * n];
    d[0][0] = 0;
    let mut q = Vec::new();
    let mut qi = 0;
    q.push((0, 0));
    while qi < q.len() {
        let (r, c) = q[qi];
        qi += 1;
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr >= (5 * n as i32) || nc < 0 || nc >= (5 * n as i32) {
                continue;
            }
            let v = {
                let wrap = (nr / (n as i32) + nc / (n as i32)) as u8;
                let orig = risk[(nr as usize) % n][(nc as usize) % n];
                let mut v = orig + wrap;
                if v > 9 {
                    v -= 9;
                }
                v as i32
            } + d[r as usize][c as usize];
            if v < d[nr as usize][nc as usize] {
                d[nr as usize][nc as usize] = v;
                q.push((nr, nc));
            }
        }
    }
    println!("part2: {}", d[5 * n - 1][5 * n - 1]);
}

fn read_input(file: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim()
        .split('\n')
        .map(|x| x.as_bytes().iter().map(|x| *x - b'0').collect())
        .collect()
}
