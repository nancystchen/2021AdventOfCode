// DUMBO OCTOPUS IS INCREDIBLY CUTE!!
//

// Part 1
// Approach: for each step
// - increase energy first
// - then, for each cell that has e > 9, propagate energy & set to 0
// - wait until there is no more e > 9.
//

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

fn parse_data(lines: Lines<BufReader<File>>) -> Vec<Vec<usize>> {
    lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| usize::from_str(&ch.to_string()).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

// Given the cave boundaries and coordinate i, j, find adjacent cells (including diagonals).
fn find_neighbours(max_i: usize, max_j: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut n: Vec<(usize, usize)> = vec![];

    // top row
    if i > 0 && j > 0 {
        n.push((i - 1, j - 1));
    }
    if i > 0 {
        n.push((i - 1, j));
    }
    if i > 0 && j < max_j {
        n.push((i - 1, j + 1));
    }

    // mid row
    if j > 0 {
        n.push((i, j - 1));
    }
    if j < max_j {
        n.push((i, j + 1));
    }

    // bottom row
    if i < max_i && j > 0 {
        n.push((i + 1, j - 1));
    }
    if i < max_i {
        n.push((i + 1, j));
    }
    if i < max_i && j < max_j {
        n.push((i + 1, j + 1));
    }

    n
}

// Feed 1 unit of energy to all octopuses. Returns total count of flashes from the feed.
fn feed_energy(octopuses: &mut [Vec<usize>]) -> usize {
    let mut flashies = vec![];
    let mut counts = 0;
    octopuses.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, o)| {
            *o += 1;
            if *o > 9 {
                flashies.push((i, j));
            }
        })
    });

    while !flashies.is_empty() {
        counts += 1;
        let (x, y) = flashies.pop().unwrap();
        octopuses[x][y] = 0;
        let neighbours = find_neighbours(octopuses.len() - 1, octopuses[0].len() - 1, x, y);
        neighbours.into_iter().for_each(|(i, j)| {
            if octopuses[i][j] > 0 {
                octopuses[i][j] += 1;

                if octopuses[i][j] > 9 && !flashies.iter().any(|(x1, y1)| *x1 == i && *y1 == j) {
                    flashies.push((i, j));
                }
            }
        });
    }

    counts
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    // or "Octopi"?
    let mut octopuses = parse_data(lines);
    /* Part 1
    let flash_counts = (0..100)
        .map(|step| {
            println!("Step {}", step);
            feed_energy(&mut octopuses)
        })
        .sum::<usize>();

    println!("Total flash counts after 100 steps: {}", flash_counts);
    */

    let mut step_count = 0;
    while feed_energy(&mut octopuses) != octopuses.len() * octopuses.len() {
        step_count += 1;
    }
    println!("First synchronous flash at step: {}", step_count + 1);
}
