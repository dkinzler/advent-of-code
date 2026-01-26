use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_input("inputs/5");
    // coordinates are in range [0, 1000)
    // we could have first iterated all lines and determined
    // the actual range of values, but this is good enough
    let mut overlap = vec![vec![0; 1000]; 1000];
    for (mut x1, mut y1, mut x2, mut y2) in lines {
        if x1 == x2 {
            if y1 > y2 {
                let tmp = y1;
                y1 = y2;
                y2 = tmp;
            }
            for i in y1..=y2 {
                overlap[x1 as usize][i as usize] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                let tmp = x1;
                x1 = x2;
                x2 = tmp;
            }
            for i in x1..=x2 {
                overlap[i as usize][y1 as usize] += 1;
            }
        }
    }

    let mut result = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if overlap[x][y] > 1 {
                result += 1;
            }
        }
    }
    println!("part1: {result}");
}

fn part2() {
    let lines = read_input("inputs/5");
    // coordinates are in range [0, 1000)
    // we could have first iterated all lines and determined
    // the actual range of values, but this is good enough
    let mut overlap = vec![vec![0; 1000]; 1000];
    for (mut x1, mut y1, mut x2, mut y2) in lines {
        if x1 == x2 {
            if y1 > y2 {
                let tmp = y1;
                y1 = y2;
                y2 = tmp;
            }
            for i in y1..=y2 {
                overlap[x1 as usize][i as usize] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                let tmp = x1;
                x1 = x2;
                x2 = tmp;
            }
            for i in x1..=x2 {
                overlap[i as usize][y1 as usize] += 1;
            }
        } else {
            // diagonal line
            let x_inc = if x1 > x2 { -1 } else { 1 };
            let y_inc = if y1 > y2 { -1 } else { 1 };
            let mut curr_x = x1;
            let mut curr_y = y1;
            while curr_x != x2 || curr_y != y2 {
                overlap[curr_x as usize][curr_y as usize] += 1;
                curr_x += x_inc;
                curr_y += y_inc;
            }
            overlap[x2 as usize][y2 as usize] += 1;
        }
    }

    let mut result = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if overlap[x][y] > 1 {
                result += 1;
            }
        }
    }
    println!("part1: {result}");
}

fn read_input(file: &str) -> Vec<(i32, i32, i32, i32)> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| {
            let (left, right) = x.split_once(" -> ").unwrap();
            let (x1, y1) = parse_point(left);
            let (x2, y2) = parse_point(right);
            (x1, y1, x2, y2)
        })
        .collect()
}

fn parse_point(s: &str) -> (i32, i32) {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}
