use std::fs;

fn main() {
    let map = read_input("inputs/25");
    let m = map.len();
    let n = map[0].len();
    let mut i = 0;
    let mut curr = map;
    loop {
        i += 1;
        let mut next = vec![vec![0; n]; m];
        let mut moves = 0;
        for r in 0..m {
            for c in 0..n {
                if curr[r][c] == 1 {
                    if curr[r][(c + 1) % n] == 0 {
                        next[r][(c + 1) % n] = 1;
                        moves += 1;
                    } else {
                        next[r][c] = 1;
                    }
                }
            }
        }
        for r in 0..m {
            for c in 0..n {
                if curr[r][c] == 2 {
                    if curr[(r + 1) % m][c] != 2 && next[(r + 1) % m][c] == 0 {
                        next[(r + 1) % m][c] = 2;
                        moves += 1;
                    } else {
                        next[r][c] = 2;
                    }
                }
            }
        }
        curr = next;
        if moves == 0 {
            break;
        }
    }
    println!("part1: {i}");
}

fn read_input(file: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim()
        .split('\n')
        .map(|x| {
            x.trim()
                .as_bytes()
                .iter()
                .map(|x| match *x {
                    b'.' => 0,
                    b'>' => 1,
                    b'v' => 2,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}
