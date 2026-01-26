use std::{collections::HashMap, fs};

fn main() {
    part1();
    part2();
}

fn part1() {
    let (adj, is_small) = read_input("inputs/12");
    // find the number of unique paths from start to end
    //
    // the graph contains small and big nodes
    // can only visit a small node once
    // and there can be no edges between two big nodes
    // because otherwise there would be an infinite number of paths
    let mut visited = vec![false; is_small.len()];
    visited[START] = true;
    let n_paths = dfs(START, &adj, &is_small, &mut visited);
    println!("part1: {n_paths}");
}

fn dfs(node: usize, adj: &Vec<Vec<usize>>, is_small: &Vec<bool>, visited: &mut Vec<bool>) -> i32 {
    if node == END {
        return 1;
    }

    let mut c = 0;
    for nb in adj[node].iter() {
        if !is_small[*nb] || !visited[*nb] {
            visited[*nb] = true;
            c += dfs(*nb, adj, is_small, visited);
            visited[*nb] = false;
        }
    }
    c
}

fn part2() {
    let (adj, is_small) = read_input("inputs/12");
    // now a single small node can be traversed twice
    let mut visited = vec![false; is_small.len()];
    visited[START] = true;
    let n_paths = dfs2(START, &adj, &is_small, &mut visited, false);
    println!("part2: {n_paths}");
}

fn dfs2(
    node: usize,
    adj: &Vec<Vec<usize>>,
    is_small: &Vec<bool>,
    visited: &mut Vec<bool>,
    used_double: bool,
) -> i32 {
    if node == END {
        return 1;
    }

    let mut c = 0;
    for nb in adj[node].iter() {
        // we can use a small node again if it is not start/end node
        // and we have not yet visited any other small node twice
        if *nb > END && is_small[*nb] && visited[*nb] && !used_double {
            c += dfs2(*nb, adj, is_small, visited, true);
        } else if !is_small[*nb] || !visited[*nb] {
            visited[*nb] = true;
            c += dfs2(*nb, adj, is_small, visited, used_double);
            visited[*nb] = false;
        }
    }
    c
}

const START: usize = 0;
const END: usize = 1;

fn read_input(file: &str) -> (Vec<Vec<usize>>, Vec<bool>) {
    let mut name_to_index = HashMap::new();
    name_to_index.insert("start".to_string(), START);
    name_to_index.insert("end".to_string(), END);

    let mut next_index = 2;

    let mut adj: Vec<Vec<usize>> = Vec::new();

    for (a, b) in fs::read_to_string(file)
        .expect("failed to read input file")
        .trim_end()
        .split('\n')
        .map(|x| {
            let (left, right) = x.trim().split_once('-').unwrap();
            (left.to_owned(), right.to_owned())
        })
    {
        let ai = get_index(&mut name_to_index, a, &mut next_index);
        let bi = get_index(&mut name_to_index, b, &mut next_index);
        if adj.len() < next_index {
            for _ in 0..(next_index - adj.len()) {
                adj.push(Vec::new());
            }
        }
        adj[ai].push(bi);
        adj[bi].push(ai);
    }

    let mut is_small = vec![false; name_to_index.len()];
    for (name, index) in name_to_index.iter() {
        match name.as_bytes()[0] {
            b'a'..=b'z' => is_small[*index] = true,
            _ => {}
        }
    }

    (adj, is_small)
}

fn get_index(h: &mut HashMap<String, usize>, name: String, next_index: &mut usize) -> usize {
    if let Some(i) = h.get(&name) {
        *i
    } else {
        h.insert(name, *next_index);
        *next_index += 1;
        *next_index - 1
    }
}
