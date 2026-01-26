use std::fs;

fn main() {
    let fish = read_input("inputs/6");
    let mut count = [0u64; 9];
    for f in fish {
        count[f] += 1;
    }

    // initially count[i] is the number of fish
    // with internal timer i
    //
    // in the next step a 0 becomes a 6 and adds an 8
    // a 1 becomes a 0, a 2 becomes a 1 and so on
    // instead of actually shifting the values we can shift the offsets
    // initially offset = 0 which means the number of fish
    // with timer 0 is stored at index 0
    // offset = k (k between 0 and 6) means the number of fish
    // with timer 0 is stored at index k
    // with each day offset increases by 1 (mod 7)
    //
    // just need to add the new fish to the correct indices
    // indices 7 and 8 are treated separately, because only new
    // fish have those timers

    let mut offset = 0;
    for i in 0..256 {
        let tmp = count[7];
        // timer 8 become timer 7
        count[7] = count[8];
        // those with timer 0 spawn one with timer 8
        count[8] = count[offset];
        // those with timer 7 become timer 6
        count[offset] += tmp;
        offset = (offset + 1) % 7;
        if i == 79 {
            let mut result = 0;
            for c in count.iter() {
                result += c;
            }
            println!("part1: {result}");
        }
    }
    let mut result = 0;
    for c in count.iter() {
        result += c;
    }
    println!("part2: {result}");
}

fn read_input(file: &str) -> Vec<usize> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}
