// Part 1
// Finding local minimum! Input is 100x100
// A simple O(n) would do if we can allocation
// this much into RAM.
//
// Part 2
// When found a local minimum, need to calculate
// the spread of basin. We can recursively search
// until we hit a `9`, which the recursion would end.
// We need to mark which locations we have visited (O(n) mem).

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

// Given a map, a set of coordinates on the map, and visited coords, visit all adjacent coords.
// Returns the total number of coords traversed.
fn visit_basin_from_coords(
    i: usize,
    j: usize,
    map: &[Vec<usize>],
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if visited.get(&(i, j)).is_some() {
        0
    } else {
        visited.insert((i, j));
        let max_i = map.len();
        let max_j = map[0].len();

        let mut nearby_grids: Vec<Option<(usize, usize)>> = vec![];
        let up = if i > 0 && map[i - 1][j] < 9 && visited.get(&(i - 1, j)).is_none() {
            Some((i - 1, j))
        } else {
            None
        };
        nearby_grids.push(up);
        let down = if i < (max_i - 1) && map[i + 1][j] < 9 && visited.get(&(i + 1, j)).is_none() {
            Some((i + 1, j))
        } else {
            None
        };
        nearby_grids.push(down);
        let left = if j > 0 && map[i][j - 1] < 9 && visited.get(&(i, j - 1)).is_none() {
            Some((i, j - 1))
        } else {
            None
        };
        nearby_grids.push(left);
        let right = if j < (max_j - 1) && map[i][j + 1] < 9 && visited.get(&(i, j + 1)).is_none() {
            Some((i, j + 1))
        } else {
            None
        };
        nearby_grids.push(right);

        1 + nearby_grids
            .into_iter()
            .filter_map(|grid| grid.map(|(x, y)| visit_basin_from_coords(x, y, map, visited)))
            .sum::<usize>()
    }
}

// Given a map, find all the local minimums
fn find_local_minimums(map: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let max_i = map.len();
    let max_j = map[0].len();
    let get_i = |idx| idx / max_j;
    let get_j = |idx| idx % max_j;
    let is_local_minimum = |i: usize, j: usize| {
        let centre = map[i][j];
        let up = if i > 0 { map[i - 1][j] } else { usize::MAX };
        let down = if i < (max_i - 1) {
            map[i + 1][j]
        } else {
            usize::MAX
        };
        let left = if j > 0 { map[i][j - 1] } else { usize::MAX };
        let right = if j < (max_j - 1) {
            map[i][j + 1]
        } else {
            usize::MAX
        };

        centre < up && centre < down && centre < left && centre < right
    };
    (0..max_i * max_j)
        .filter_map(|idx| {
            let i: usize = get_i(idx);
            let j: usize = get_j(idx);
            if is_local_minimum(i, j) {
                Some((i, j))
            } else {
                None
            }
        })
        .collect()
}

// Given lines, create a map
fn parse_data(lines: Lines<BufReader<File>>) -> Vec<Vec<usize>> {
    lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let map = parse_data(lines);
    let local_minimums = find_local_minimums(&map);
    println!("local minimums: {:?}", local_minimums);
    let sum_of_risk_levels = local_minimums.len()
        + local_minimums
            .iter()
            .map(|(i, j)| map[*i][*j])
            .sum::<usize>();
    println!("Total risk level: {}", sum_of_risk_levels);
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut basin_sizes = local_minimums
        .iter()
        .map(|lm| visit_basin_from_coords(lm.0, lm.1, &map, &mut visited))
        .collect::<Vec<usize>>();
    println!("Basin size: {:?}", basin_sizes);
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let basin_product = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
    println!("Three biggest basin product {}", basin_product);
}
