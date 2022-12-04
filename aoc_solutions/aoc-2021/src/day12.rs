use std::collections::{HashMap, HashSet};

type CaveNetwork<'a> = HashMap<&'a str, Vec<&'a str>>;

fn is_lower(s: &str) -> bool {
    s.chars().next().unwrap().is_lowercase()
}

fn parse_input(input: &String) -> CaveNetwork {
    let mut network = HashMap::new();

    for line in input.lines() {
        let (c1, c2) = line.split_once("-").unwrap();
        let c1_neighbors: &mut Vec<&str> = network.entry(c1).or_insert(Vec::new());
        c1_neighbors.push(c2);
        let c2_neighbors: &mut Vec<&str> = network.entry(c2).or_insert(Vec::new());
        c2_neighbors.push(c1);
    }
    network
}

pub fn solve_first(input: String) {
    let network = parse_input(&input);
    let mut visited = HashSet::new();
    let n_paths = visit_first("start", &network, &mut visited);
    println!("n_paths: {n_paths}");
}

pub fn solve_second(input: String) {
    let network = parse_input(&input);
    let mut visited = HashSet::new();
    let n_paths = visit_second("start", &network, &mut visited);
    println!("n_paths: {n_paths}");
}

fn visit_first<'a>(
    cave: &'a str,
    network: &'a CaveNetwork,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if cave == "end" {
        return 1;
    }
    // if this is a small cave we've visited, go back
    if visited.contains(cave) {
        0
    } else {
        // this cave has not been visited
        // if this is a small cave or start, set it as visited
        if cave == "start" || is_lower(cave) {
            visited.insert(cave);
        }
        // and then recurse to all its neighbors
        let mut sum = 0;
        for next_cave in network.get(cave).unwrap().iter() {
            sum += visit_first(*next_cave, network, visited);
        }
        // before returning up the stack track, set the current node as unvisited again such that other paths might visit it
        visited.remove(cave);
        sum
    }
}
fn visit_second<'a>(
    cave: &'a str,
    network: &'a CaveNetwork,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if cave == "end" {
        return 1;
    }
    // if this is a small cave we've visited, go back
    if visited.contains(cave) {
        0
    } else {
        // this cave has not been visited
        // if this is a small cave or start, set it as visited
        if cave == "start" || is_lower(cave) {
            visited.insert(cave);
        }
        // and then recurse to all its neighbors
        let mut sum = 0;
        for next_cave in network.get(cave).unwrap().iter() {
            sum += visit_first(*next_cave, network, visited);
        }
        // before returning up the stack track, set the current node as unvisited again such that other paths might visit it
        visited.remove(cave);
        sum
    }
}
