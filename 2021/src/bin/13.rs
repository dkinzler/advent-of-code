use std::{collections::HashSet, fs};

fn main() {
    let (points, folds) = read_input("inputs/13");
    let mut curr = points;
    let mut unique_next: HashSet<(i32, i32)> = HashSet::new();
    for (i, fold) in folds.iter().enumerate() {
        unique_next.clear();
        let mut next = Vec::new();
        match *fold {
            Fold::X(v) => {
                for (x, y) in curr.iter() {
                    let z;
                    if *x < v {
                        z = (*x, *y);
                    } else {
                        let d = *x - v;
                        z = (v - d, *y);
                    }
                    if !unique_next.contains(&z) {
                        next.push(z);
                        unique_next.insert(z);
                    }
                }
            }
            Fold::Y(v) => {
                for (x, y) in curr.iter() {
                    let z;
                    if *y < v {
                        z = (*x, *y);
                    } else {
                        let d = *y - v;
                        z = (*x, v - d);
                    }
                    if !unique_next.contains(&z) {
                        next.push(z);
                        unique_next.insert(z);
                    }
                }
            }
        }
        if i == 0 {
            println!("part1: {}", next.len());
        }
        curr = next;
    }

    let max_x = curr.iter().map(|x| x.0).max().unwrap();
    let max_y = curr.iter().map(|x| x.1).max().unwrap();

    for y in 0..=max_y {
        let mut row = String::new();
        for x in 0..=max_x {
            if unique_next.contains(&(x, y)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        println!("{row}");
    }
}

enum Fold {
    X(i32),
    Y(i32),
}

fn read_input(file: &str) -> (Vec<(i32, i32)>, Vec<Fold>) {
    let contents = fs::read_to_string(file).expect("failed to read input file");
    let (points, folds) = contents.trim_end().split_once("\n\n").unwrap();

    let points = points
        .trim()
        .split('\n')
        .map(|x| {
            let (left, right) = x.trim().split_once(',').unwrap();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .collect();

    let folds = folds
        .trim()
        .trim()
        .split('\n')
        .map(|x| {
            let (left, right) = x.trim_start_matches("fold along ").split_once('=').unwrap();
            let v = right.parse::<i32>().unwrap();
            if left.as_bytes()[0] == b'x' {
                Fold::X(v)
            } else {
                Fold::Y(v)
            }
        })
        .collect();

    (points, folds)
}
