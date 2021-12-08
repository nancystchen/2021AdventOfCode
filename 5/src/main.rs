use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const VENT_HEIGHT_LIMIT: usize = 2;

#[derive(Debug)]
struct VentLine {
    start: (u32, u32),
    end: (u32, u32),
}

impl VentLine {
    fn forms_horizontal_lines(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn forms_vertical_lines(&self) -> bool {
        self.start.0 == self.end.0
    }
}

// Given a data map of vents, count how many vents exceeds the height maximum
fn count_tall_vents(map_data: &HashMap<(u32, u32), usize>, height: usize) -> usize {
    map_data.values().filter(|count| **count >= height).count()
}

// Given a start/end, generate a vector of coords
// The coords will be generated regardless of order of start/end
fn generate_coords(vent_data: VentLine) -> Vec<(u32, u32)> {
    if vent_data.forms_horizontal_lines() {
        if vent_data.start.0 < vent_data.end.0 {
            (vent_data.start.0..(vent_data.end.0 + 1))
                .map(|x| (x, vent_data.start.1))
                .collect::<Vec<(u32, u32)>>()
        } else {
            (vent_data.end.0..(vent_data.start.0 + 1))
                .map(|x| (x, vent_data.start.1))
                .collect::<Vec<(u32, u32)>>()
        }
    } else if vent_data.forms_vertical_lines() {
        if vent_data.start.1 < vent_data.end.1 {
            (vent_data.start.1..(vent_data.end.1 + 1))
                .map(|y| (vent_data.start.0, y))
                .collect::<Vec<(u32, u32)>>()
        } else {
            (vent_data.end.1..(vent_data.start.1 + 1))
                .map(|y| (vent_data.start.0, y))
                .collect::<Vec<(u32, u32)>>()
        }
    } else {
        let x_sequence: Vec<u32>  = {
            if vent_data.start.0 < vent_data.end.0 {
                (vent_data.start.0..(vent_data.end.0 + 1)).collect()
            } else {
                (vent_data.end.0..(vent_data.start.0 + 1)).rev().collect()
            }
        };
        let y_sequence: Vec<u32> = {
            if vent_data.start.1 < vent_data.end.1 {
                (vent_data.start.1..(vent_data.end.1 + 1)).collect()
            } else {
                (vent_data.end.1..(vent_data.start.1 + 1)).rev().collect()
            }
        };
        x_sequence.into_iter().zip(y_sequence.into_iter()).collect()
    }
}

// Given a data string, create a vent line
// Sample data:
// `39,28 -> 39,846`
fn parse_data(data: String) -> VentLine {
    let parsed_coords = data
        .replace(" -> ", ",")
        .split(',')
        .map(|coord_str| coord_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    VentLine {
        start: (parsed_coords[0], parsed_coords[1]),
        end: (parsed_coords[2], parsed_coords[3]),
    }
}

fn main() {
    println!("Solving problem...");
    let mut data_map = HashMap::<(u32, u32), usize>::new();
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    lines.for_each(|line| {
        if let Ok(data) = line {
            let vent_line = parse_data(data);
            generate_coords(vent_line).into_iter().for_each(|coord| {
                let counter = data_map.entry(coord).or_insert(0);
                *counter += 1;
            });
        }
    });
    let tall_vent_count = count_tall_vents(&data_map, VENT_HEIGHT_LIMIT);
    println!(
        "Number of vents equal or greater than {}: {}",
        VENT_HEIGHT_LIMIT, tall_vent_count
    );
}
