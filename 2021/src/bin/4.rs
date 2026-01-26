use std::fs;

fn main() {
    let (seq, mut boards) = read_input("inputs/4");
    let n = boards.len();
    let mut winners = 0;
    let mut won = vec![false; n];
    for x in seq.iter() {
        for (i, board) in boards.iter_mut().enumerate() {
            if !won[i] {
                board.mark(*x);
                if board.is_winner() {
                    won[i] = true;

                    if winners == 0 {
                        let result = board.score() * (*x);
                        println!("part1: {result}");
                    } else if winners == n - 1 {
                        let result = board.score() * (*x);
                        println!("part2: {result}");
                        return;
                    }
                    winners += 1;
                }
            }
        }
    }
}

struct Board {
    nums: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn new(s: Vec<String>) -> Board {
        let mut nums = [[0; 5]; 5];
        let marked = [[false; 5]; 5];
        for r in 0..5 {
            let v = s[r]
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap());

            for (c, x) in v.enumerate() {
                nums[r][c] = x;
            }
        }
        Board { nums, marked }
    }

    fn mark(&mut self, num: i32) {
        for r in 0..5 {
            for c in 0..5 {
                if self.nums[r][c] == num {
                    self.marked[r][c] = true;
                }
            }
        }
    }
    fn is_winner(&self) -> bool {
        for i in 0..5 {
            let mut row = true;
            let mut col = true;
            for j in 0..5 {
                row &= self.marked[i][j];
                col &= self.marked[j][i];
            }
            if row || col {
                return true;
            }
        }
        return false;
    }

    fn score(&self) -> i32 {
        let mut sum = 0;
        for r in 0..5 {
            for c in 0..5 {
                if !self.marked[r][c] {
                    sum += self.nums[r][c];
                }
            }
        }
        sum
    }
}

fn read_input(file: &str) -> (Vec<i32>, Vec<Board>) {
    let contents = fs::read_to_string(file).expect("failed to read input file");

    let lines = contents.trim_end().split('\n').collect::<Vec<&str>>();

    let seq = lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::new();

    let mut i = 2;

    while i < lines.len() {
        let mut s = Vec::new();
        for j in 0..5 {
            s.push(lines[i + j].to_owned());
        }
        i += 6;
        boards.push(Board::new(s));
    }
    (seq, boards)
}
