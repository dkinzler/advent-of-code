fn main() {
    // the MONAD program consists of 14 smaller programs
    // each of which reads in a digit and processes it
    // it reads the digit into w
    // x and y are temporary variables, they are reset every time
    // using mul x/y, 0
    // it changes the value in z and that carries over into following iterations
    //
    // each iteration does roughly the same thing
    // x = z % 26
    // z = z / 1 or z = z / 26 (there might be a pattern behind this?)
    // x += c    where c changes between iterations
    //           and can be negative
    // eq x, w   x = 1 if x==w else 0
    // eq x, 0   the result of previous eq won't matter
    //           if x == 0 -> set x = 1 else set x = 0
    //           together these two implement neq x, w
    // y = 25 * x + 1   where x is either 0 or 1
    //                  so y will be either 1 or 26
    // z *= y
    // y = (w + d) * x  where d changes between iterations
    //                  will be either w+d or 0 depending on x
    //                  d values are relatively small, largest is 14?
    //                  and w is from 1 to 9
    // add z y          will add either w+d or 0
    //
    //
    // or lets write it as a small program
    // v = (z_old % 26) + c
    // if v == w:
    //    z_new = z_old / 26
    //              or
    //            z_old
    // else:
    //    z_new = (z_old / 26) * 26 + w + d
    //              or
    //            z_old*26 + w + d
    //
    //
    // initially z = 0, each iteration calculates a new value of z
    // at the end the z value needs to be 0 for the number
    // to be valid
    // there are two types of steps
    // type 0 does z = z/1
    // type 1 does z = z/26
    // there are 7 steps of each type
    //
    // for type 0 steps the c constant is >= 10 every time so we
    // can't get v==w
    // in that case we always compute z_new = z_old*26+w+d
    // we can try any digit w for these steps
    //
    // for type 1 steps the c constant is often < 0 so we can get
    // v == w
    // with v=w we compute z_new = z_old / 26
    //
    // type 0 steps increase the value of z by multiplying by 26
    // and adding w + d
    // type 1 steps with v=w decrease z by dividing by 26
    //
    // so we roughly need an equal amount of both steps
    // to end up with z = 0
    //
    // naively we can choose 14 digits with 9 possibilites for each
    // that is a huge search space
    // but because we need to eventually reach z=0 we will have
    // to choose some digits with v==w -> so there is only one choice there
    // which decreases the search space substantially
    //
    // we can just implement a backtracking function that tries
    // one digit after another
    // but we can eliminate some nodes early
    // if at any step we have k type 1 steps left
    // then we can get the current z value down to at most
    // m = z / 26 / 26 ... / 26 (k times)
    // if m > 0 we cannot reach z=0 at the last step and we can
    // return early
    let mut choices = [0; 14];
    let result = backtrack(0, 0, 7, &mut choices);
    println!("{:?}", result);
}

const STEP_TYPE: [i64; 14] = [0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1];
const C: [i64; 14] = [14, 12, 11, -4, 10, 10, 15, -9, -9, 12, -15, -7, -10, 0];
const D: [i64; 14] = [7, 4, 8, 1, 5, 14, 12, 10, 5, 7, 6, 8, 4, 6];

fn backtrack(z: i64, i: usize, type_1_remaining: i64, choices: &mut [i64; 14]) -> Option<i64> {
    // v = (z_old % 26) + c
    // if v == w:
    //    z_new = z_old / 26
    //              or
    //            z_old
    // else:
    //    z_new = (z_old / 26) * 26 + w + d
    //              or
    //            z_old*26 + w + d
    // base case
    if i == 14 {
        if z == 0 {
            let mut result = 0;
            for i in 0..14 {
                result = result * 10 + choices[i];
            }
            return Some(result);
        } else {
            return None;
        }
    }
    if i >= 7 {
        // compute bound
        let mut m = z;
        for _ in 0..type_1_remaining {
            m = m / 26;
        }
        if m > 0 {
            return None;
        }
    }
    let v = (z % 26) + C[i];
    // for part 1 we want the maximum possible value
    // so we have to try the digits in reverse order
    // for w in (1..=9).rev() {
    for w in 1..=9 {
        choices[i] = w;
        if STEP_TYPE[i] == 0 {
            if v == w {
                panic!("this shouldn't happen");
            }
            if let Some(z) = backtrack(z * 26 + w + D[i], i + 1, type_1_remaining, choices) {
                return Some(z);
            }
        } else {
            let new_z;
            if v == w {
                new_z = z / 26;
            } else {
                new_z = (z / 26) * 26 + w + D[i];
            }
            if let Some(z) = backtrack(new_z, i + 1, type_1_remaining - 1, choices) {
                return Some(z);
            }
        }
    }
    return None;
}
