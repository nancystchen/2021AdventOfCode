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
    // if cave is small and has also been visited, returns empty vec
    // else use adj_table to find adj nodes:
    //   for each of the adj nodes that are not start:
    //     path = find_end(adj_node_name, adj_table, path + name + ',')
    // return paths.flattened() 

    unimplemented!()
}

fn main() {
    // parse data
    let file = std::fs::File::open("input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    // create adjacency table
    let mut adjacency_table = HashMap::<String, Vec<String>>::new();

    // for each line:
    //   create connections in table( end doesnt have neighbours, start is not in values )
    let paths = find_end("start", &adjacency_table, "");
    println!("Total paths found: {}", paths.len());
}
