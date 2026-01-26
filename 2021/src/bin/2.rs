use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input("inputs/2");
    let mut h = 0;
    let mut d = 0;
    for c in input.iter() {
        match c {
            Command::Forward(v) => {
                h += v;
            }
            Command::Up(v) => {
                d -= v;
            }
            Command::Down(v) => {
                d += v;
            }
        }
    }

    let result = h * d;
    println!("part1: {result}");
}

fn part2() {
    let input = read_input("inputs/2");
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    for c in input.iter() {
        match c {
            Command::Forward(v) => {
                h += v;
                d += aim * v;
            }
            Command::Up(v) => {
                aim -= v;
            }
            Command::Down(v) => {
                aim += v;
            }
        }
    }

    let result = h * d;
    println!("part1: {result}");
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn read_input(file: &str) -> Vec<Command> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| {
            let mut parts = x.split(' ');
            let dir = parts.next().unwrap();
            let d = parts.next().unwrap().parse::<i32>().unwrap();

            match dir {
                "forward" => Command::Forward(d),
                "up" => Command::Up(d),
                "down" => Command::Down(d),
                _ => panic!("unexpected command: {}", dir),
            }
        })
        .collect()
}
