use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut grid = read_input("inputs/11");
    let n = 10;
    let mut q = Vec::new();
    let mut flashes = 0;
    for _ in 0..100 {
        for r in 0..n {
            for c in 0..n {
                let v = grid[r][c];
                if v + 1 == 10 {
                    q.push((r, c));
                    grid[r][c] = 0;
                    flashes += 1;
                } else {
                    grid[r][c] = v + 1;
                }
            }
        }
        let mut qi = 0;
        while qi < q.len() {
            let (r, c) = q[qi];
            qi += 1;
            for dr in [-1, 0, 1] {
                for dc in [-1, 0, 1] {
                    let nr = (r as i32) + dr;
                    let nc = (c as i32) + dc;
                    if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;
                    match grid[nr][nc] {
                        0 => {}
                        9 => {
                            q.push((nr, nc));
                            grid[nr][nc] = 0;
                            flashes += 1;
                        }
                        v => {
                            grid[nr][nc] = v + 1;
                        }
                    }
                }
            }
        }
        q.clear();
    }
    println!("part1: {flashes}");
}

fn part2() {
    let mut grid = read_input("inputs/11");
    let n = 10;
    let mut q = Vec::new();
    for step in 0..1000 {
        let mut flashes = 0;
        for r in 0..n {
            for c in 0..n {
                let v = grid[r][c];
                if v + 1 == 10 {
                    q.push((r, c));
                    grid[r][c] = 0;
                    flashes += 1;
                } else {
                    grid[r][c] = v + 1;
                }
            }
        }
        let mut qi = 0;
        while qi < q.len() {
            let (r, c) = q[qi];
            qi += 1;
            for dr in [-1, 0, 1] {
                for dc in [-1, 0, 1] {
                    let nr = (r as i32) + dr;
                    let nc = (c as i32) + dc;
                    if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;
                    match grid[nr][nc] {
                        0 => {}
                        9 => {
                            q.push((nr, nc));
                            grid[nr][nc] = 0;
                            flashes += 1;
                        }
                        v => {
                            grid[nr][nc] = v + 1;
                        }
                    }
                }
            }
        }
        if flashes == n * n {
            println!("part2: {}", step + 1);
            return;
        }
        q.clear();
    }
}

fn read_input(file: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| x.as_bytes().iter().map(|x| *x - b'0').collect())
        .collect()
}
