use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let mut scanners = read_input("inputs/19");
    let n_scanner = scanners.len();

    let mut transforms = Vec::new();

    for xi in 0..3 {
        for xv in [-1, 1] {
            let x = Point {
                x: if xi == 0 { xv } else { 0 },
                y: if xi == 1 { xv } else { 0 },
                z: if xi == 2 { xv } else { 0 },
            };
            for yi in 0..3 {
                if xi == yi {
                    continue;
                }
                for yv in [-1, 1] {
                    let y = Point {
                        x: if yi == 0 { yv } else { 0 },
                        y: if yi == 1 { yv } else { 0 },
                        z: if yi == 2 { yv } else { 0 },
                    };
                    let z = Point {
                        x: x.y * y.z - x.z * y.y,
                        y: x.z * y.x - x.x * y.z,
                        z: x.x * y.y - x.y * y.x,
                    };

                    let transform = [[x.x, y.x, z.x], [x.y, y.y, z.y], [x.z, y.z, z.z]];
                    transforms.push(transform);
                }
            }
        }
    }

    let mut diffs = Vec::new();
    for scanner in scanners.iter() {
        diffs.push(compute_diffs(scanner));
    }

    let mut found = vec![false; n_scanner];
    let mut q = Vec::new();
    q.push(0);
    found[0] = true;
    let mut qi = 0;
    let mut origins = Vec::new();
    origins.push(Point { x: 0, y: 0, z: 0 });
    while q.len() < n_scanner {
        // there should be a solution, so qi >= q.len() shouldn't happen
        let a = q[qi];
        qi += 1;
        for b in 0..n_scanner {
            if !found[b] {
                if let Some(origin) = find_overlap(&mut scanners, &mut diffs, &transforms, a, b) {
                    found[b] = true;
                    q.push(b);
                    origins.push(origin);
                }
            }
        }
    }

    let mut beacons = HashSet::new();
    for scanner in scanners.iter() {
        for point in scanner.iter() {
            beacons.insert(*point);
        }
    }
    println!("part1: {}", beacons.len());

    let mut max_dist = 0;
    for i in 0..origins.len() - 1 {
        let a = origins[i];
        for j in i + 1..origins.len() {
            let b = origins[j];
            let dist = (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    println!("part2: {}", max_dist);
}

fn find_overlap(
    scanners: &mut Vec<Vec<Point>>,
    diffs: &mut Vec<Vec<(Point, usize, usize)>>,
    transforms: &Vec<[[i64; 3]; 3]>,
    a: usize,
    b: usize,
) -> Option<Point> {
    let mut diff_map: HashMap<Point, (usize, usize)> = HashMap::new();
    for (diff, i, j) in diffs[a].iter() {
        diff_map.insert(*diff, (*i, *j));
    }

    for transform in transforms.iter() {
        let mut matches = 0;
        let mut one_match = None;
        for (diff, i, j) in diffs[b].iter() {
            let t = diff.apply_transform(transform);
            if diff_map.contains_key(&t) {
                matches += 1;
                one_match = Some((t, *i, *j));
            }
        }
        if matches >= 12 * 11 {
            let (diff, i, _) = one_match.unwrap();
            let (other_i, _) = diff_map.get(&diff).unwrap();
            let point1 = scanners[a][*other_i];
            let point2 = scanners[b][i].apply_transform(transform).flip();
            let origin = point1.add(&point2);

            let mut points = Vec::new();
            for p in scanners[b].iter() {
                let new_p = origin.add(&p.apply_transform(transform));
                points.push(new_p);
            }
            let new_diff = compute_diffs(&points);
            scanners[b] = points;
            diffs[b] = new_diff;
            println!("found transform: {origin:?}");
            return Some(origin);
        }
    }
    return None;
}

fn compute_diffs(x: &Vec<Point>) -> Vec<(Point, usize, usize)> {
    let mut result = Vec::new();
    let n = x.len();
    for i in 0..n - 1 {
        let a = &x[i];
        for j in i + 1..n {
            let b = &x[j];
            let mut diff = a.sub(&b);
            let mut first_index = j;
            let mut second_index = i;
            if diff.x < 0 {
                diff = diff.flip();
                first_index = i;
                second_index = j;
            } else if diff.x == 0 && diff.y < 0 {
                diff = diff.flip();
                first_index = i;
                second_index = j;
            } else if diff.x == 0 && diff.y == 0 && diff.z < 0 {
                diff = diff.flip();
                first_index = i;
                second_index = j;
            }
            result.push((diff, first_index, second_index));
            result.push((diff.flip(), second_index, first_index));
        }
    }
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn flip(&self) -> Point {
        Point {
            x: -1 * self.x,
            y: -1 * self.y,
            z: -1 * self.z,
        }
    }

    fn apply_transform(&self, t: &[[i64; 3]; 3]) -> Point {
        Point {
            x: t[0][0] * self.x + t[0][1] * self.y + t[0][2] * self.z,
            y: t[1][0] * self.x + t[1][1] * self.y + t[1][2] * self.z,
            z: t[2][0] * self.x + t[2][1] * self.y + t[2][2] * self.z,
        }
    }
}

fn read_input(file: &str) -> Vec<Vec<Point>> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .skip(1)
                .map(|x| {
                    let p: Vec<i64> = x.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
                    Point {
                        x: p[0],
                        y: p[1],
                        z: p[2],
                    }
                })
                .collect()
        })
        .collect()
}
