use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input("inputs/1");
    let mut result = 0;
    for (i, v) in input.iter().enumerate() {
        if i > 0 && *v > input[i - 1] {
            result += 1;
        }
    }
    println!("part1: {result}");
}

fn part2() {
    let input = read_input("inputs/1");
    let n = input.len();
    let input: Vec<i32> = input
        .iter()
        .enumerate()
        .map_while(|(j, x)| {
            if j + 2 >= n {
                None
            } else {
                Some(*x + input[j + 1] + input[j + 2])
            }
        })
        .collect();

    let mut result = 0;
    for (i, v) in input.iter().enumerate() {
        if i > 0 && *v > input[i - 1] {
            result += 1;
        }
    }
    println!("part2: {result}");
}

fn read_input(file: &str) -> Vec<i32> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
