use std::{collections::HashMap, fs};

fn main() {
    part1();
    part2();
}

fn part1() {
    let (template, rules) = read_input("inputs/14");

    let mut r: HashMap<(u8, u8), u8> = HashMap::new();
    for (a, b) in rules {
        r.insert(a, b);
    }

    let mut curr = template;
    for _ in 0..10 {
        let mut next = Vec::new();
        next.push(curr[0]);
        for i in 1..curr.len() {
            next.push(*r.get(&(curr[i - 1], curr[i])).unwrap());
            next.push(curr[i]);
        }
        curr = next;
    }

    let mut count = [0; 26];
    for b in curr {
        count[(b - b'A') as usize] += 1;
    }

    let mut min = 1 << 30;
    let mut max = 0;
    for v in count {
        if v > 0 {
            if v > max {
                max = v;
            }
            if v < min {
                min = v;
            }
        }
    }
    println!("part1: {} {} {}", min, max, max - min);
}

fn part2() {
    let (template, rules) = read_input("inputs/14");

    // because of the exponential growth need a faster solution
    // instead of actually tracking the string we can just
    // track the number of pairs for each possible pair
    // e.g. all pairs AB will yield a new pair AC and CB
    // if the rule is AB->C
    // we can just do this with hash maps, that is easy to code
    // although not the fastest

    let mut r: HashMap<(u8, u8), u8> = HashMap::new();
    for (a, b) in rules {
        r.insert(a, b);
    }

    let mut pairs: HashMap<(u8, u8), u64> = HashMap::new();
    for i in 1..template.len() {
        let pair = (template[i - 1], template[i]);
        if let Some(v) = pairs.get_mut(&pair) {
            *v += 1;
        } else {
            pairs.insert(pair, 1);
        }
    }

    let mut curr = pairs;
    for _ in 0..40 {
        let mut next: HashMap<(u8, u8), u64> = HashMap::new();
        for (pair, v) in curr {
            let c = *r.get(&pair).unwrap();
            let left = (pair.0, c);
            if let Some(z) = next.get_mut(&left) {
                *z += v;
            } else {
                next.insert(left, v);
            }
            let right = (c, pair.1);
            if let Some(z) = next.get_mut(&right) {
                *z += v;
            } else {
                next.insert(right, v);
            }
        }
        curr = next;
    }

    let mut count = [0u64; 26];
    for ((a, b), c) in curr {
        count[(a - b'A') as usize] += c;
        count[(b - b'A') as usize] += c;
    }

    // with this we count every element twice because it appears
    // in two pairs, except the first and last letter

    let mut min = 1 << 60;
    let mut max = 0;
    let start = (template[0] - b'A') as usize;
    let end = (template.last().unwrap() - b'A') as usize;
    for (c, v) in count.iter().enumerate() {
        if *v > 0 {
            let z;
            if c == start || c == end {
                z = (*v + 1) / 2;
            } else {
                z = *v / 2;
            }
            if z > max {
                max = z;
            }
            if z < min {
                min = z;
            }
        }
    }
    println!("part2: {} {} {}", min, max, max - min);
}

fn read_input(file: &str) -> (Vec<u8>, Vec<((u8, u8), u8)>) {
    let contents = fs::read_to_string(file).expect("failed to read input file");
    let (template, rules) = contents.trim_end().split_once("\n\n").unwrap();

    let template = template.trim().as_bytes().to_owned();

    let rules = rules
        .trim()
        .split('\n')
        .map(|x| {
            let (left, right) = x.split_once(" -> ").unwrap();
            let left = (left.as_bytes()[0], left.as_bytes()[1]);
            let right = right.as_bytes()[0];
            (left, right)
        })
        .collect();

    (template, rules)
}
