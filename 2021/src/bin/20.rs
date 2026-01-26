use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (alg, image) = read_input("inputs/20");
    // initially all the pixels outside the image on the infinite plane
    // are 0
    // but any pixel whose 3x3 window is all 0 will get transformed into 1
    // because for our input the alg value of 0 is 1
    // that means in the first step all the pixels in the entire infinite plane
    // outside the image will be turned on
    // but then in the next step they will all be turned off again
    // that is why we have the outside_value parameter
    let image = apply_alg(&alg, &image, 0);
    let image = apply_alg(&alg, &image, 1);

    let mut on_count = 0;
    for r in 0..image.len() {
        for c in 0..image.len() {
            if image[r][c] == 1 {
                on_count += 1;
            }
        }
    }
    println!("part1: {on_count}");
}

fn part2() {
    let (alg, image) = read_input("inputs/20");
    // same thing as in part 1 but apply algorithm 50 times
    // the outside values will flip between 0 and 1 each time
    let mut curr = image;
    for i in 0..50 {
        curr = apply_alg(&alg, &curr, i % 2);
    }

    let mut on_count = 0;
    for r in 0..curr.len() {
        for c in 0..curr.len() {
            if curr[r][c] == 1 {
                on_count += 1;
            }
        }
    }
    println!("part2: {on_count}");
}

fn apply_alg(alg: &Vec<u8>, image: &Vec<Vec<u8>>, outside_value: u8) -> Vec<Vec<u8>> {
    // applying the filter essentially adds 1 additional row/column
    // on both sides, so both width and height increase by 2
    // all other pixels outside of that on the inifinte plane will have the same value

    let n = image.len();
    let mut output = vec![vec![0u8; n + 2]; n + 2];
    for r in 0..n + 2 {
        for c in 0..n + 2 {
            let mut alg_index = 0usize;
            for i in 0..9 {
                let dr = (i / 3) - 1;
                let dc = (i % 3) - 1;

                let nr = (r as i32) + dr;
                let nc = (c as i32) + dc;
                let v = if nr <= 0 || nr >= ((n + 1) as i32) || nc <= 0 || nc >= ((n + 1) as i32) {
                    outside_value
                } else {
                    image[(nr - 1) as usize][(nc - 1) as usize]
                };
                if v == 1 {
                    alg_index |= 1 << (8 - i)
                }
            }
            output[r][c] = alg[alg_index];
        }
    }
    output
}

fn read_input(file: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let contents = fs::read_to_string(file).expect("failed to read input file");
    let (alg, image) = contents.trim_end().split_once("\n\n").unwrap();

    let alg = alg
        .trim()
        .as_bytes()
        .iter()
        .map(|x| match *x {
            b'.' => 0,
            b'#' => 1,
            _ => panic!(),
        })
        .collect();

    let image = image
        .trim()
        .split('\n')
        .map(|x| {
            x.trim()
                .as_bytes()
                .iter()
                .map(|x| match *x {
                    b'.' => 0,
                    b'#' => 1,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    (alg, image)
}
