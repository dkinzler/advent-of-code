use std::fs;

fn main() {
    part1();
}

fn part1() {
    let positions = read_input("inputs/7");
    let max_pos = *positions.iter().max().unwrap();

    let mut min_cost_1 = 1 << 30;
    let mut min_cost_2 = 1 << 30;
    for i in 1..max_pos {
        let mut cost_1 = 0;
        let mut cost_2 = 0;
        for x in positions.iter() {
            let steps = if i > *x { i - (*x) } else { (*x) - i };
            cost_1 += steps;
            cost_2 += (steps + 1) * steps / 2;
        }
        if cost_1 < min_cost_1 {
            min_cost_1 = cost_1;
        }
        if cost_2 < min_cost_2 {
            min_cost_2 = cost_2;
        }
    }
    println!("part1: {min_cost_1}");
    println!("part2: {min_cost_2}");
}

fn read_input(file: &str) -> Vec<i32> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
