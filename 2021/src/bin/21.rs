use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    // input:
    // Player 1 starting position: 7
    // Player 2 starting position: 3

    // instead of using positions 1 to 10
    // we use 0 to 9 to be able to use modular arithemtic
    let mut pos1 = 6;
    let mut pos2 = 2;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut dice = 0;
    let mut roll = || {
        dice += 1;
        if dice > 100 {
            dice = 1;
        }
        dice
    };

    let mut rolls = 0;
    loop {
        let r1 = roll() + roll() + roll();
        pos1 = (pos1 + r1) % 10;
        score1 += pos1 + 1;
        rolls += 3;
        if score1 >= 1000 {
            println!("part1: {}", score2 * rolls);
            return;
        }

        let r2 = roll() + roll() + roll();
        pos2 = (pos2 + r2) % 10;
        score2 += pos2 + 1;
        rolls += 3;
        if score2 >= 1000 {
            println!("part1: {}", score1 * rolls);
            return;
        }
    }
}

fn part2() {
    // input:
    // Player 1 starting position: 7
    // Player 2 starting position: 3

    // now play until one player reaches at least 21
    // with a three-sided quantum die
    // for both players compute the number of universes in which
    // they win
    // a state can be described fully by the positions and scores
    // of both players, but there are multiple ways to reach
    // each state
    // we could compute those recursively with memoization?
    let mut combinations = [0u64; 10];
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                combinations[d1 + d2 + d3] += 1;
            }
        }
    }

    let mut mem = HashMap::new();
    // player 1 wins
    let mut p1 = 0;
    // winning score can be at most 30
    // because when score is 20 and player hits position 10
    for score1 in 21..=30 {
        for score2 in 0..21 {
            for pos1 in 0..10 {
                for pos2 in 0..10 {
                    p1 += rec(pos1, pos2, score1, score2, &mut mem, &combinations);
                }
            }
        }
    }
    // player 2 wins
    let mut p2 = 0;
    for score2 in 21..=30 {
        for score1 in 0..21 {
            for pos1 in 0..10 {
                for pos2 in 0..10 {
                    p2 += rec(pos1, pos2, score1, score2, &mut mem, &combinations);
                }
            }
        }
    }
    println!("part2:\nplayer 1: {}\nplayer 2: {}", p1, p2);
}

fn rec(
    pos1: u8,
    pos2: u8,
    score1: u8,
    score2: u8,
    mem: &mut HashMap<(u8, u8, u8, u8), u64>,
    combinations: &[u64; 10],
) -> u64 {
    if let Some(v) = mem.get(&(pos1, pos2, score1, score2)) {
        return *v;
    }

    if score1 == 0 && score2 == 0 {
        if pos1 == 6 && pos2 == 2 {
            return 1;
        } else {
            return 0;
        }
    }

    let mut r = 0;
    if score1 >= 21 {
        // player 1 won, only need to make a move for it
        // p1 could have rolled either 1, 2 or 3 to get here
        for i in 3..=9 {
            let prev_pos1 = (pos1 + 10 - i) % 10;
            if score1 < pos1 + 1 {
                continue;
            }
            let prev_score1 = score1 - pos1 - 1;
            // previous score needs to be < 21
            // otherwise game would have ended already in previous move
            if prev_score1 >= 21 {
                continue;
            }
            r += combinations[i as usize]
                * rec(prev_pos1, pos2, prev_score1, score2, mem, combinations);
        }
    } else {
        // we can make a move for both players
        for d1 in 3..=9 {
            let prev_pos1 = (pos1 + 10 - d1) % 10;
            if score1 < pos1 + 1 {
                continue;
            }
            let prev_score1 = score1 - pos1 - 1;
            for d2 in 3..=9 {
                let prev_pos2 = (pos2 + 10 - d2) % 10;
                if score2 < pos2 + 1 {
                    continue;
                }
                let prev_score2 = score2 - pos2 - 1;
                if score2 >= 21 && prev_score2 >= 21 {
                    // if player 2 won, previous score
                    // must have been below 21, otherwise
                    // game would have already been over
                    continue;
                }
                r += combinations[d1 as usize]
                    * combinations[d2 as usize]
                    * rec(
                        prev_pos1,
                        prev_pos2,
                        prev_score1,
                        prev_score2,
                        mem,
                        combinations,
                    );
            }
        }
    }
    mem.insert((pos1, pos2, score1, score2), r);
    return r;
}
