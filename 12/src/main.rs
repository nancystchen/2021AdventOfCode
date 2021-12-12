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
fn find_end(name: &str, adj_table: &HashMap<String, Vec<String>>, path: &str) -> Vec<String> {
    // if end, return [path + "end"]
    if name == "end" {
        vec![concat!(path, name)]
    } else if is_small_cave(name) && visited(path, name) {
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
            .iter(|adj| find_end(adj, adj_table, concat!(path, adj, ",")))
            .flatten()
            .collect
    }
}

fn visited(name: &str) -> bool {
    unimplemented!()
}
fn is_small_cave(name: &str) -> bool {
    unimplemented!()
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
            // ensure end doesn't have values, start is not in values
            let node_one_adj = adjacency_table
                .entry(node_one.to_string())
                .or_insert(vec![]);
            if node_two != &"start" || node_one != &"end" {
                node_one_adj.push(node_two.to_string());
            }
            let node_two_adj = adjacency_table
                .entry(node_two.to_string())
                .or_insert(vec![]);
            if node_one != &"start" || node_two != &"end" {
                node_two_adj.push(node_one.to_string());
            }
        };
    });
    let paths = find_end("start", &adjacency_table, "");
    println!("Total paths found: {}", paths.len());
}
