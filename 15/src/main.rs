// Famous Dijkstra's algorithm question! Improved using BinaryHeap.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn dijkstras_algo(source: (usize, usize), des: (usize, usize), graph: &[Vec<usize>]) -> usize {
    // create a dist table des_i * des_j to mark distances.
    // set dist[source] = 0, other entries usize::MAX.
    let mut dist = vec![vec![usize::MAX; des.1 + 1]; des.0 + 1];
    dist[source.0][source.1] = 0;
    // create another to_visit queue to host nodes to visit
    let mut to_visit = BinaryHeap::new();
    // starting from source as current node, for its neighbours:
    to_visit.push(Reverse((0, (source.0, source.1))));
    // - if haven't visited all neighbours
    while let Some(Reverse((_, curr))) = to_visit.pop() {
        // - update node neighbours' distances from source if smaller.
        let neighbours = get_neighbours(curr.0, curr.1, graph[0].len(), graph.len());
        neighbours.iter().for_each(|n| {
            let n_dist = dist[curr.0][curr.1] + graph[n.0][n.1];
            if n_dist < dist[n.0][n.1] {
                dist[n.0][n.1] = n_dist;
                // if value is updated, we need to check its neighbours again.
                // a cycle won't be added because cycle cost more than its subparts.
                to_visit.push(Reverse((dist[n.0][n.1], (n.0, n.1))));
            }
        });
    }
    // return dist[des] value
    dist[des.0][des.1]
}

// we can create graph on the fly because of the problem type
fn get_neighbours(i: usize, j: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];
    // if has cell below
    if i > 0 {
        n.push((i - 1, j));
    }
    // if has cell above
    if i + 1 < rows {
        n.push((i + 1, j));
    }
    // if has cell to the left
    if j > 0 {
        n.push((i, j - 1));
    }
    // if has cell to the right
    if j + 1 < cols {
        n.push((i, j + 1));
    }
    n
}

fn parse_data(data: Lines<BufReader<File>>) -> Vec<Vec<usize>> {
    data.map(|line| {
        line.unwrap()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>()
    })
    .collect()
}

// 5x dimension of given graph
// TODO: can just use inner logic to calculate value to reduce mem usage.
fn make_full_graph(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let part_width = graph[0].len();
    let part_height = graph.len();
    let full_width = graph[0].len() * 5;
    let full_height = graph.len() * 5;
    (0..full_width)
        .map(|i| {
            (0..full_height)
                .map(|j| {
                    let grid = i / part_width + j / part_height;
                    let old_val = graph[i % part_width][j % part_height];
                    let mut new_val = old_val + grid;
                    if new_val != 9 {
                        new_val %= 9;
                    }
                    new_val
                })
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn main() {
    let file = File::open("sample_input.txt").unwrap();
    let data = BufReader::new(file).lines();
    let graph = parse_data(data);
    let shortest_path = dijkstras_algo((0, 0), (graph[0].len() - 1, graph.len() - 1), &graph);
    println!("Lowest risk level: {}", shortest_path);

    let full_graph = make_full_graph(&graph);
    let shortest_full_path = dijkstras_algo(
        (0, 0),
        (full_graph[0].len() - 1, full_graph.len() - 1),
        &full_graph,
    );
    println!("Lowest risk level in full map: {}", shortest_full_path);
}
