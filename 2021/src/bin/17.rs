fn main() {
    // input:
    // target area: x=244..303, y=-91..-54
    let x_start = 244;
    let x_end = 303;
    let y_start = -91;
    let y_end = -54;

    // does the x value even matter?
    // we can always make it hit the range, but how high the
    // y goes is entirely dependent only on y?
    // if initial dy=y
    // then we move y+(y-1)+(y-2)+...+1 steps up
    // that is the highest point
    // and then move down in steps of 1, 2, 3, 4, ... y
    // then we are back at 0
    // so y must be < -1*y_start because otherwise it will
    // overshoot in one shot?
    // after we reach 0 height again, will move y+1 steps
    // then y+2 and so on
    // but choosing abs(y_start)-1 is optimal because then
    // we hit the target area right at the end with a single step

    let z = (y_start * -1) - 1;
    println!("part1: {}", z * (z + 1) / 2);

    // for part2 we want the number of distinct initial velocity pairs
    // that eventually hit the target area
    // we can't analyze x and y separately
    // but we can just simulate, there are not that many possible solutions

    // any higher will instantly overshoot
    let mut result = 0;
    for vx in 1..=x_end {
        // any lower will instantly overshoot
        // any higher will also overshoot
        for vy in y_start..(-1 * y_start) {
            if sim(vx, vy, x_start, x_end, y_start, y_end) {
                result += 1;
            }
        }
    }
    println!("part2: {}", result);
}

// if a probe fired with the given initial velocities
// hits the target area, this function returns the max height it reaches
// otherwise returns None
fn sim(vx: i32, vy: i32, x_start: i32, x_end: i32, y_start: i32, y_end: i32) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let mut vy = vy;
    loop {
        x += vx;
        y += vy;
        if x_start <= x && x <= x_end && y_start <= y && y <= y_end {
            return true;
        }
        if x > x_end || y < y_start {
            return false;
        }
        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }
        vy -= 1;
    }
}
