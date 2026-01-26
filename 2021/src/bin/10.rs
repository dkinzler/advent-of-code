use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_input("inputs/10");
    let points = [3, 57, 1197, 25137];
    let mut result = 0;
    for line in lines {
        let mut stack = Vec::new();
        for b in line.as_bytes() {
            let (v, is_open) = match *b {
                b'(' => (0, true),
                b')' => (0, false),
                b'[' => (1, true),
                b']' => (1, false),
                b'{' => (2, true),
                b'}' => (2, false),
                b'<' => (3, true),
                b'>' => (3, false),
                _ => panic!("unexpected char"),
            };
            if is_open {
                stack.push(v);
            } else {
                if let Some(o) = stack.pop() {
                    if o != v {
                        result += points[v];
                        break;
                    }
                } else {
                    result += points[v];
                    break;
                }
            }
        }
    }
    println!("part1: {result}");
}

fn part2() {
    let lines = read_input("inputs/10");
    let mut completion_scores = Vec::new();
    for line in lines {
        let mut stack = Vec::new();
        let mut is_incomplete = true;
        for b in line.as_bytes() {
            let (v, is_open) = match *b {
                b'(' => (0, true),
                b')' => (0, false),
                b'[' => (1, true),
                b']' => (1, false),
                b'{' => (2, true),
                b'}' => (2, false),
                b'<' => (3, true),
                b'>' => (3, false),
                _ => panic!("unexpected char"),
            };
            if is_open {
                stack.push(v);
            } else {
                if let Some(o) = stack.pop() {
                    if o != v {
                        is_incomplete = false;
                        break;
                    }
                } else {
                    is_incomplete = false;
                    break;
                }
            }
        }
        if !is_incomplete {
            continue;
        }
        let mut score = 0i64;
        while let Some(o) = stack.pop() {
            score = score * 5 + o + 1;
        }
        completion_scores.push(score);
    }
    completion_scores.sort();
    let result = completion_scores[completion_scores.len() / 2];

    println!("part2: {result}");
}

fn read_input(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| x.to_owned())
        .collect()
}
