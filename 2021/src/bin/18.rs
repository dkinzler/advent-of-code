use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let numbers = read_input("inputs/18");
    let result = numbers.into_iter().reduce(|a, x| add(&a, &x)).unwrap();
    println!("part1: {}", magnitude(&result));
}

fn part2() {
    let numbers = read_input("inputs/18");
    let n = numbers.len();
    let mut max = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let r = magnitude(&add(&numbers[i], &numbers[j]));
            if r > max {
                max = r;
            }
            let r = magnitude(&add(&numbers[j], &numbers[i]));
            if r > max {
                max = r;
            }
        }
    }
    println!("part2: {}", max);
}

fn add(a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut x = Vec::new();
    x.extend_from_slice(a);
    x.extend_from_slice(b);
    for (_, depth) in x.iter_mut() {
        *depth += 1;
    }

    loop {
        if !try_explode(&mut x) {
            if !try_split(&mut x) {
                break;
            }
        }
    }
    x
}

fn try_explode(a: &mut Vec<(i32, i32)>) -> bool {
    let mut z = None;
    for (i, (_, depth)) in a.iter().enumerate() {
        if *depth == 5 {
            z = Some(i);
            break;
        }
    }
    if let Some(i) = z {
        let left = a[i].0;
        let right = a[i + 1].0;
        if i > 0 {
            a[i - 1].0 += left;
        }
        if i + 2 < a.len() {
            a[i + 2].0 += right;
        }
        a[i] = (0, 4);
        a.remove(i + 1);
        return true;
    }
    false
}

fn try_split(a: &mut Vec<(i32, i32)>) -> bool {
    let mut z = None;
    for (i, (value, _)) in a.iter().enumerate() {
        if *value >= 10 {
            z = Some(i);
            break;
        }
    }
    if let Some(i) = z {
        let value = a[i].0;
        let depth = a[i].1;
        let left = value / 2;
        let right = if value % 2 == 1 {
            (value + 1) / 2
        } else {
            value / 2
        };
        a[i] = (left, depth + 1);
        a.insert(i + 1, (right, depth + 1));
        return true;
    }
    false
}

fn magnitude(a: &Vec<(i32, i32)>) -> i32 {
    let mut stack = Vec::new();
    stack.push(a[0]);
    for (value, depth) in a.iter().skip(1) {
        let mut curr_value = *value;
        let mut curr_depth = *depth;
        loop {
            if stack.is_empty() {
                break;
            }
            let (top_value, top_depth) = *stack.last().unwrap();
            if top_depth == curr_depth {
                stack.pop();
                curr_value = 3 * top_value + 2 * curr_value;
                curr_depth -= 1;
            } else {
                break;
            }
        }
        stack.push((curr_value, curr_depth));
    }
    stack.last().unwrap().0
}

fn read_input(file: &str) -> Vec<Vec<(i32, i32)>> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim()
        .split('\n')
        .map(|x| parse_number(x))
        .collect()
}

fn parse_number(s: &str) -> Vec<(i32, i32)> {
    let mut number = Vec::new();
    let mut depth = 0;
    for b in s.as_bytes() {
        match *b {
            b'[' => depth += 1,
            b']' => depth -= 1,
            b',' => {}
            b'0'..=b'9' => number.push(((*b - b'0') as i32, depth)),
            _ => panic!(),
        }
    }
    number
}
