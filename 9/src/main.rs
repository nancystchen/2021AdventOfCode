// Finding local minimum! Input is 100x100
// A simple O(n) would do if we can allocation
// this much into RAM.
//
//
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

// Given a map, find all the local minimums
fn find_local_minimums(map: &Vec<Vec<usize>>) -> Vec<usize> {
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
                Some(map[i][j])
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
    let sum_of_risk_levels = local_minimums.len() + local_minimums.iter().sum::<usize>();
    println!("Total risk level: {}", sum_of_risk_levels);
}
