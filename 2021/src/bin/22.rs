use std::{
    cmp::{max, min},
    fs,
};

fn main() {
    part1();
    part2();
}

// note we could have operated directly on cubes
// and computed cube intersections and cube subtractions
// but this was probably less error prone and still runs
// reasonably fast

fn part1() {
    // we can iterate over z coordinates, so we consider
    // one slice of the big cube at a time
    // then we have a set of rectangles defined by x + y ranges
    // e.g. x=10..20 y = -4..6
    //
    // then we could compute intersections to find the set of
    // cubes that are on
    // how do we do that?
    // suppose we have two rectangles a and b and want to remove
    // the intersection c of both from a
    // we have a = ax1..ax2 ay1..ay2
    // we have c = cx1..cx2 cy1..cy2
    // the difference can be expressed as at most 4 new
    // disjoint rectangles
    // 1: ax1..cx1-1 cy1..cy2
    // 2: cx2+1..ax2 cy1..cy2
    // 3: ax1..ax2   ay1..cy1-1
    // 4: ax1..ax2   cy2+1..ay2
    //
    // might have to intersect all of these again with a
    // to get the part that is actually in
    //
    // we keep the set of on cubes that are disjoint
    // for an off step we
    // iterate over all the element in the set and remove the off
    // section via the method above
    // this produces at most 4 new rectangles each
    //
    // for an on step we want to make sure that all the areas
    // in the set are disjoint
    // how do we do this?
    // let assume the new cube is b
    // iterate over each element a in the set
    // if intersection of a and b is not empty, remove b from a
    // add all the new rectangles into a new set
    // in the end add b to the set
    // at the end we just need to sum the volumes of all the cubes in the set
    let steps = read_input("inputs/22");
    let steps: Vec<Step> = steps
        .into_iter()
        .filter(|x| {
            let c = match x {
                Step::On(c) => c,
                Step::Off(c) => c,
            };
            c.x_start >= -50
                && c.x_end <= 50
                && c.y_start >= -50
                && c.y_end <= 50
                && c.z_start >= -50
                && c.z_end <= 50
        })
        .collect();

    println!("part1: {}", compute_on(steps));
}

fn part2() {
    let steps = read_input("inputs/22");
    println!("part1: {}", compute_on(steps));
}

fn compute_on(steps: Vec<Step>) -> i64 {
    let z_min = steps
        .iter()
        .map(|x| match x {
            Step::On(c) => c.z_start,
            Step::Off(c) => c.z_start,
        })
        .min()
        .unwrap();
    let z_max = steps
        .iter()
        .map(|x| match x {
            Step::On(c) => c.z_end,
            Step::Off(c) => c.z_end,
        })
        .max()
        .unwrap();

    let mut result = 0;
    for z in z_min..=z_max {
        let mut rects = Vec::new();
        for step in steps.iter() {
            if !match step {
                Step::On(c) => c.z_start <= z && z <= c.z_end,
                Step::Off(c) => c.z_start <= z && z <= c.z_end,
            } {
                continue;
            }
            let mut next = Vec::new();
            match step {
                Step::On(c) => {
                    let b = c.to_xy_rect();
                    for r in rects {
                        let x = subtract(r, &b);
                        next.extend(x);
                    }
                    next.push(b);
                }
                Step::Off(c) => {
                    let b = c.to_xy_rect();
                    for r in rects {
                        if let Some(i) = intersect(&r, &b) {
                            let x = subtract(r, &i);
                            next.extend(x);
                        } else {
                            next.push(r);
                        }
                    }
                }
            }
            rects = next;
        }
        for r in rects.iter() {
            result += r.area();
        }
    }
    result
}

struct Rect {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
}

impl Rect {
    fn is_empty(&self) -> bool {
        self.x_start > self.x_end || self.y_start > self.y_end
    }

    fn area(&self) -> i64 {
        ((self.x_end - self.x_start + 1) as i64) * ((self.y_end - self.y_start + 1) as i64)
    }
}

fn intersect(a: &Rect, b: &Rect) -> Option<Rect> {
    let r = Rect {
        x_start: max(a.x_start, b.x_start),
        x_end: min(a.x_end, b.x_end),
        y_start: max(a.y_start, b.y_start),
        y_end: min(a.y_end, b.y_end),
    };
    if !r.is_empty() { Some(r) } else { None }
}

fn subtract(a: Rect, b: &Rect) -> Vec<Rect> {
    let mut result = Vec::new();
    if let Some(c) = intersect(&a, b) {
        let l1 = Rect {
            x_start: a.x_start,
            x_end: c.x_start - 1,
            y_start: c.y_start,
            y_end: c.y_end,
        };
        if !l1.is_empty() {
            result.push(l1);
        }

        let l2 = Rect {
            x_start: c.x_end + 1,
            x_end: a.x_end,
            y_start: c.y_start,
            y_end: c.y_end,
        };
        if !l2.is_empty() {
            result.push(l2);
        }

        let l3 = Rect {
            x_start: a.x_start,
            x_end: a.x_end,
            y_start: a.y_start,
            y_end: c.y_start - 1,
        };
        if !l3.is_empty() {
            result.push(l3);
        }

        let l4 = Rect {
            x_start: a.x_start,
            x_end: a.x_end,
            y_start: c.y_end + 1,
            y_end: a.y_end,
        };
        if !l4.is_empty() {
            result.push(l4);
        }
    } else {
        result.push(a);
    }
    result
}

#[derive(Debug)]
struct Cube {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
}

impl Cube {
    fn to_xy_rect(&self) -> Rect {
        Rect {
            x_start: self.x_start,
            x_end: self.x_end,
            y_start: self.y_start,
            y_end: self.y_end,
        }
    }
}

#[derive(Debug)]
enum Step {
    On(Cube),
    Off(Cube),
}

fn read_input(file: &str) -> Vec<Step> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim()
        .split('\n')
        .map(|x| {
            let y;
            let on;
            if x.starts_with("on ") {
                on = true;
                y = x.trim_start_matches("on ")
            } else {
                on = false;
                y = x.trim_start_matches("off ")
            }

            let ranges: Vec<(i32, i32)> = y
                .split(',')
                .map(|x| {
                    let (_, right) = x.split_once('=').unwrap();
                    let (left, right) = right.split_once("..").unwrap();
                    (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
                })
                .collect();

            let cube = Cube {
                x_start: ranges[0].0,
                x_end: ranges[0].1,
                y_start: ranges[1].0,
                y_end: ranges[1].1,
                z_start: ranges[2].0,
                z_end: ranges[2].1,
            };

            if on { Step::On(cube) } else { Step::Off(cube) }
        })
        .collect()
}
