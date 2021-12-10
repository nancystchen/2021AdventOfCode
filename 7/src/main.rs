// New way of importing file content! Courtesy of https://github.com/pk-nb
static INPUT_FILE: &str = include_str!("../input.txt");

fn parse_data(data: &str) -> Vec<usize> {
    let mut v = data
        .trim()
        .split(',')
        .map(|point| point.parse().unwrap())
        .collect::<Vec<usize>>();
    v.sort_unstable();
    v
}

// Approach is the following:
// sort the data, and for each of the position between (min, max],
// we calculate the fuel usage.
// if at every step we use the calculation from previous step, the
// computation becomes quite trivial at O(n) + O(nlogn).

fn part_one() {
    let ordered_data_vec = parse_data(INPUT_FILE);
    let min_value = ordered_data_vec.iter().min().unwrap_or(&0);
    let max_value = ordered_data_vec.iter().max().unwrap_or(&0);
    // defatuls to using pos 0
    let mut best_pos = 0;
    let mut data_index = 0;
    let mut least_fuel: usize = ordered_data_vec.iter().sum();

    let total_num = ordered_data_vec.len();
    let mut current_fuel = least_fuel;

    ((min_value + 1)..(max_value + 1)).for_each(|point| {
        while ordered_data_vec[data_index] < point {
            data_index += 1;
        }
        let ge_point_num = total_num - data_index;
        let l_point_num = total_num - ge_point_num;

        // for all the points greater than/equal to pos, we move one unit closer ;
        // for the ones less, we moved one unit farther away.
        let fuel_for_point = current_fuel - ge_point_num + l_point_num;
        if fuel_for_point < least_fuel {
            best_pos = point;
            least_fuel = fuel_for_point;
        }
        current_fuel = fuel_for_point;
    });

    println!("least fuel {} at pos {}", least_fuel, best_pos);
}

// Less elegant O(n^2) solution to calculate fuel comsumption for each possible position
// by summing up each crab submarine's fuel usage :|
fn part_two() {
    let ordered_data_vec = parse_data(INPUT_FILE);

    let calculate_fuel_usage = |distance: usize| distance * (distance + 1) / 2;
    // defatuls to using pos 0
    let mut best_pos = 0;

    let mut least_fuel: usize = ordered_data_vec
        .iter()
        .map(|pos| calculate_fuel_usage(*pos))
        .sum();

    let min_value = ordered_data_vec.iter().min().unwrap_or(&0);
    let max_value = ordered_data_vec.iter().max().unwrap_or(&0);

    ((min_value + 1)..(max_value + 1)).for_each(|point| {
        let fuel_for_point = ordered_data_vec
            .iter()
            .map(|&d| calculate_fuel_usage((point as i32 - d as i32).abs() as usize))
            .sum();
        if fuel_for_point < least_fuel {
            least_fuel = fuel_for_point;
            best_pos = point;
        }
    });

    println!("least fuel {} at pos {}", least_fuel, best_pos);
}

fn main() {
    part_one();
    part_two();
}
