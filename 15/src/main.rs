// Famous Dijkstra's algorithm question!

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn dijkstras_algo(source: (usize, usize), des: (usize, usize), graph: &[Vec<usize>]) -> usize {
    // create a dist table des_i * des_j to mark distances.
    // set dist[source] = 0, other entries usize::MAX.
    let mut dist = vec![vec![usize::MAX; des.1 + 1]; des.0 + 1];
    dist[source.0][source.1] = 0;
    // create another visited table dis_i * des_j to mark visits.
    let mut visited = vec![vec![false; des.1 + 1]; des.0 + 1];
    // starting from source as current node, for its neighbours:
    let mut curr = (source.0, source.1);
    // - if haven't found des node (problem gurantees foundable des)
    while !visited[des.0][des.1] {
        // - update node neighbours' distances from source if smaller.
        let neighbours = get_neighbours(curr.0, curr.1, graph);
        neighbours.iter().for_each(|n| {
            let n_dist = dist[curr.0][curr.1] + graph[n.0][n.1];
            if n_dist < dist[n.0][n.1] {
                dist[n.0][n.1] = n_dist
            }
        });

        // - mark current node as visited
        visited[curr.0][curr.1] = true;
        // - find node with the smallest value in dist[] that's unvisited
        curr = find_unvisited_min(&dist, &visited);
    }
    // return dist[des] value
    dist[des.0][des.1]
}

fn find_unvisited_min(dist: &[Vec<usize>], v: &[Vec<bool>]) -> (usize, usize) {
    let mut min = usize::MAX;
    let mut min_i = 0;
    let mut min_j = 0;
    dist.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, val)| {
            if *val < min && !v[i][j] {
                min = *val;
                min_i = i;
                min_j = j;
            };
        })
    });
    (min_i, min_j)
}

// we can create graph on the fly because of the problem type - only right or down.
fn get_neighbours(i: usize, j: usize, graph: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut n = vec![];
    if i + 1 < graph[0].len() {
        n.push((i + 1, j));
    }
    if j + 1 < graph.len() {
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

fn main() {
    let file = File::open("input.txt").unwrap();
    let data = BufReader::new(file).lines();
    let graph = parse_data(data);
    let shortest_path = dijkstras_algo((0, 0), (graph[0].len() - 1, graph.len() - 1), &graph);
    println!("Lowest risk level: {}", shortest_path);
}
