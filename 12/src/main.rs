// Part 1:
// Approach:
// - we construct a graph from data.
// - we traverse from `start`, via `find`
//  - ends when
//   - `end` is found (returns path)
//   - double small cave is found (returns none)
// - to all the neighboring nodes

use std::collections::HashMap;
use std::io::BufRead;

// Given a node name, find the path to `end`.
// Returns Some(path) if possible. If not returns None.
fn find_end(
    name: &str,
    adj_table: &HashMap<String, Vec<String>>,
    path: &str,
    visit_small_cave_twice: bool,
) -> Vec<String> {
    // if end, return [path + "end"]
    if name == "end" {
        vec![format!("{},{}", path, name)]
    } else if is_small_cave(name)
        && (if visit_small_cave_twice {
            visited_two_caves_twice_or_one_cave_three_times(path)
        } else {
            visited_cave_twice(path, name)
        })
    {
        // if cave is small and has also been visited, returns empty vec
        vec![]
    } else {
        // else use adj_table to find adj nodes:
        //   for each of the adj nodes that are not start:
        //     path = find_end(adj_node_name, adj_table, path + name + ',')
        // return paths.flattened()
        adj_table
            .get(name)
            .unwrap()
            .iter()
            .map(|adj| {
                find_end(
                    adj,
                    adj_table,
                    &format!("{},{}", path, adj),
                    visit_small_cave_twice,
                )
            })
            .flatten()
            .collect()
    }
}

// Given a path and a small cave name, check if we have already visited the cave twice.
fn visited_cave_twice(path: &str, name: &str) -> bool {
    path.split(',')
        .filter(|node_name| *node_name == name)
        .count()
        > 1
}

// Given a path, check if we visited more than one small cave twice, or a single small cave three
// times.
fn visited_two_caves_twice_or_one_cave_three_times(path: &str) -> bool {
    let small_cave_counts = path
        .split(',')
        .fold(HashMap::<&str, usize>::new(), |mut acc, node_name| {
            if is_small_cave(node_name) {
                let count = acc.entry(node_name).or_insert(0);
                *count += 1;
            }
            acc
        })
        .into_values()
        .collect::<Vec<usize>>();
    small_cave_counts.iter().filter(|n| **n == 2).count() > 1
        || small_cave_counts.iter().any(|n| *n == 3)
}

// Check if a cave is a small cave by name.
fn is_small_cave(name: &str) -> bool {
    name != "start" && name != "end" && name.chars().next().unwrap().is_lowercase()
}

fn main() {
    // parse data
    let file = std::fs::File::open("input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    // create adjacency table
    let mut adjacency_table = HashMap::<String, Vec<String>>::new();

    // for each line:
    //   create connections in table
    lines.for_each(|line| {
        if let Ok(data_string) = line {
            let nodes = data_string.split('-').collect::<Vec<&str>>();
            let node_one = nodes.first().unwrap();
            let node_two = nodes.last().unwrap();
            // if node is already present, add the connection
            // otherwise, add node, then connection
            // ensure end doesn't have any connectison, and start is not a connection to nodes
            let node_one_adj = adjacency_table
                .entry(node_one.to_string())
                .or_insert_with(std::vec::Vec::new);
            if *node_two != "start" && *node_one != "end" {
                node_one_adj.push(node_two.to_string());
            }
            let node_two_adj = adjacency_table
                .entry(node_two.to_string())
                .or_insert_with(std::vec::Vec::new);
            if *node_one != "start" && *node_two != "end" {
                node_two_adj.push(node_one.to_string());
            }
        };
    });
    let paths_part_1 = find_end("start", &adjacency_table, "start", false);
    println!("Part 1: Total paths found: {}", paths_part_1.len());
    let paths_part_2 = find_end("start", &adjacency_table, "start", true);
    println!("Part 2: Total paths found: {}", paths_part_2.len());
}
