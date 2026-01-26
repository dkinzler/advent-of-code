use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let map = read_input("inputs/9");
    let m = map.len() as i32;
    let n = map[0].len() as i32;
    let mut result = 0;
    for r in 0..m {
        for c in 0..n {
            let mut is_low_point = true;
            for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= m || nc < 0 || nc >= n {
                    continue;
                }
                if map[nr as usize][nc as usize] <= map[r as usize][c as usize] {
                    is_low_point = false;
                    break;
                }
            }
            if is_low_point {
                result += map[r as usize][c as usize] + 1;
            }
        }
    }

    println!("part1: {result}");
}

fn part2() {
    let map = read_input("inputs/9");
    let m = map.len() as i32;
    let n = map[0].len() as i32;

    let mut basin_map = vec![vec![0; n as usize]; m as usize];
    let mut basin = 1;
    let mut basin_sizes = Vec::new();
    for r in 0..m {
        for c in 0..n {
            let mut is_low_point = true;
            for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= m || nc < 0 || nc >= n {
                    continue;
                }
                if map[nr as usize][nc as usize] <= map[r as usize][c as usize] {
                    is_low_point = false;
                    break;
                }
            }
            if is_low_point {
                basin_sizes.push(find_basin_size(&map, &mut basin_map, r, c, basin));
                basin += 1;
            }
        }
    }

    basin_sizes.sort();
    let z = basin_sizes.len();
    let result = basin_sizes[z - 1] * basin_sizes[z - 2] * basin_sizes[z - 3];
    println!("part2: {result}");
}

fn find_basin_size(
    map: &Vec<Vec<i32>>,
    basin_map: &mut Vec<Vec<i32>>,
    start_r: i32,
    start_c: i32,
    basin: i32,
) -> i32 {
    let m = map.len() as i32;
    let n = map[0].len() as i32;
    let mut q = Vec::new();
    q.push((start_r, start_c));
    let mut qi = 0;

    basin_map[start_r as usize][start_c as usize] = basin;
    let mut size = 1;

    while qi < q.len() {
        let (r, c) = q[qi];
        qi += 1;
        let v = map[r as usize][c as usize];
        for (dr, dc) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr >= m || nc < 0 || nc >= n {
                continue;
            }
            if basin_map[nr as usize][nc as usize] == basin {
                continue;
            }
            let nv = map[nr as usize][nc as usize];
            if nv < 9 && nv >= v {
                size += 1;
                basin_map[nr as usize][nc as usize] = basin;
                q.push((nr, nc));
            }
        }
    }
    size
}

fn read_input(file: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| {
            let mut row = Vec::new();
            for b in x.as_bytes() {
                row.push((*b - b'0') as i32);
            }
            row
        })
        .collect()
}
