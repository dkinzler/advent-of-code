use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input("inputs/3");
    // strings in input all have the same length
    let k = input[0].len();
    let n = input.len();
    let mut one_count = vec![0; k];
    for x in input.iter() {
        for (i, b) in x.as_bytes().iter().enumerate() {
            if *b == b'1' {
                one_count[i] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, c) in one_count.iter().enumerate() {
        if *c > n - *c {
            // most common bit is 1
            gamma += 1 << (k - 1 - i);
        } else {
            // most common bit is 0
            epsilon += 1 << (k - 1 - i);
        }
    }

    let power = gamma * epsilon;
    println!("part1: {power}");
}

fn part2() {
    let input = read_input("inputs/3");
    // strings in input all have the same length
    let k = input[0].len();

    let oxygen_gen_rating = filter(&input, k, true);
    let co2_scrub_rating = filter(&input, k, false);
    let life_support_rating = oxygen_gen_rating * co2_scrub_rating;
    println!("part2: {life_support_rating}");
}

fn filter(input: &Vec<String>, k: usize, most_common: bool) -> i32 {
    let mut curr: Vec<usize> = (0..input.len()).collect();
    let mut pos = 0;
    while curr.len() > 1 {
        let mut next = Vec::new();

        let n = curr.len();
        let mut one_count = 0;
        for x in curr.iter() {
            if input[*x].as_bytes()[pos] == b'1' {
                one_count += 1;
            }
        }

        let to_keep = if most_common {
            if one_count >= n - one_count {
                b'1'
            } else {
                b'0'
            }
        } else {
            if one_count < n - one_count {
                b'1'
            } else {
                b'0'
            }
        };

        for x in curr.iter() {
            if input[*x].as_bytes()[pos] == to_keep {
                next.push(*x);
            }
        }

        curr = next;
        pos += 1;
    }

    let mut result = 0;
    for i in 0..k {
        if input[curr[0] as usize].as_bytes()[i] == b'1' {
            result += 1 << (k - 1 - i);
        }
    }
    result
}

fn read_input(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| x.to_owned())
        .collect()
}
