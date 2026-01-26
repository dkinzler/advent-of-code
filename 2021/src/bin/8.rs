use std::fs;

fn main() {
    part1();
    // this code quickly became kind of a mess, but oh well, it works I guess
    part2();
}

fn part1() {
    let displays = read_input("inputs/8");
    let mut result = 0;
    for (_, outputs) in displays.iter() {
        for output in outputs {
            result += match output.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }
    println!("part1: {result}");
}

fn part2() {
    let mut displays = read_input("inputs/8");
    let real_nums: Vec<u8> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .map(|x| str_to_pattern(x))
    .collect();
    // the deduction process for all displays will be exactly the same
    // the length 2 pattern corresponds to digit 1
    // the length 3 pattern corresponds to digit 7
    // -> the letter that is in 7 but not 1 corresponds to segment a
    // the letters in 8 but not 1,4,7 correspond to e, g
    // lets consider the digits of length 5 which are 2, 3, 5
    // if we take away all the letters from 1, 4, 7, 8
    // what remains can only be e, g
    // g appears in all 3, e only in one
    // so we know a, e, g at this point
    // the letter that appears only in 2, but not 7 and is not e or g
    // must be d
    // so we have a, d, e, g
    // there is one pattern of length 5 = digit 2
    // that has a, d, e, g + 1, that unknown one is c
    // from there we can find out f and so on
    // but how do we code this?
    // encode the pattern as bit sets and sort them by length?

    let mut result = 0;
    for (patterns, outputs) in displays.iter_mut() {
        patterns.sort_by_key(|x| x.len());
        let patterns = parse_patterns(patterns);
        // maps letters in the pattern to their actual segment
        let mut mapping = [0usize; 7];

        let a = get_bit_indices(patterns[1] & !patterns[0])[0];
        mapping[a] = 0;
        let all_147 = patterns[0] | patterns[1] | patterns[2];
        let intersect_235 = patterns[3] & patterns[4] & patterns[5];
        let all_235 = patterns[3] | patterns[4] | patterns[5];
        // we want all the bits in 1,4,7 gone, what remains must be g
        let g = get_bit_indices(intersect_235 & (!all_147))[0];
        // if we remove from the bits in 2,3,5 all the bits from 1,4,7,8 and g
        // what remains must be e
        let e = get_bit_indices(all_235 & (!(all_147 | (1 << g))))[0];
        mapping[g] = 6;
        mapping[e] = 4;

        let pattern_2 = if has_bit(patterns[3], e) {
            patterns[3]
        } else if has_bit(patterns[4], e) {
            patterns[4]
        } else {
            patterns[5]
        };

        let d = get_bit_indices(pattern_2 & (!patterns[1]) & !(1 << e) & !(1 << g))[0];
        mapping[d] = 3;
        let adeg = (1 << a) | (1 << d) | (1 << e) | (1 << g);
        let c = get_bit_indices(pattern_2 & !adeg)[0];
        mapping[c] = 2;

        // 1 only has segments c and f
        let f = get_bit_indices(patterns[0] & !(1 << c))[0];
        mapping[f] = 5;

        // 4 has b, c, d, f
        let b = get_bit_indices(patterns[2] & !(1 << c) & !(1 << d) & !(1 << f))[0];
        mapping[b] = 1;

        let outputs = parse_patterns(outputs);
        let mut z = 0;
        for output in outputs.iter() {
            let v = find_number(&real_nums, &mapping, *output);
            z = z * 10 + v;
        }
        result += z;
    }
    println!("part2: {result}");
}

fn parse_patterns(p: &mut Vec<String>) -> Vec<u8> {
    p.iter().map(|x| str_to_pattern(x)).collect()
}

fn str_to_pattern(s: &str) -> u8 {
    let mut v = 0;
    for b in s.as_bytes() {
        let i = b - b'a';
        v |= 1 << i
    }
    v
}

fn find_number(real_nums: &[u8], mapping: &[usize; 7], pattern: u8) -> i32 {
    let mut z = 0;
    for i in 0..7 {
        if pattern & (1 << i) > 0 {
            z |= 1 << mapping[i];
        }
    }
    for (i, v) in real_nums.iter().enumerate() {
        if *v == z {
            return i as i32;
        }
    }
    panic!("number not found");
}

fn get_bit_indices(x: u8) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..=7 {
        if x & (1 << i) > 0 {
            result.push(i);
        }
    }
    return result;
}

fn has_bit(x: u8, i: usize) -> bool {
    x & (1 << i) > 0
}

fn read_input(file: &str) -> Vec<(Vec<String>, Vec<String>)> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| {
            let (left, right) = x.split_once('|').unwrap();
            let patterns: Vec<String> = left
                .trim()
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect();

            let outputs: Vec<String> = right
                .trim()
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect();

            (patterns, outputs)
        })
        .collect()
}
