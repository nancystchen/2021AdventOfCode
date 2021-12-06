use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn from_binary_str_to_decimal(binary_str: &str) -> i32 {
    unimplemented!()
}

fn calculate_epsilon_rate(gemma_rate: &str) -> String {
    unimplemented!()
}

fn calculate_gemma_rate(data: Lines<BufReader<File>>) -> String {
    let mut lines = data
        .peekable();
    let mut num_one_in_binary_position: Vec<usize> = lines
        .peek()
        .expect("Cannot find the first line of data").as_ref()
        .map(|line| {
            vec![0; line.len()]
        })
        .unwrap();
    let mut total_line_count = 0;
    lines.enumerate().for_each(|(idx, line)| {
        let binary_str: String = line.expect("Cannot parse data into binary string");
        binary_str.chars().enumerate().for_each(|(idx, c)| {
            if c == '1' {
                num_one_in_binary_position[idx] += 1;
            }
        });
        total_line_count = idx;
    });
    let gemma_rate =
        num_one_in_binary_position
            .into_iter()
            .fold(String::new(), |mut acc, count| {
                if count > (total_line_count / 2) {
                    acc.push('1');
                    acc
                } else {
                    acc.push('0');
                    acc
                }
            });

    gemma_rate
}

fn get_data(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).expect("Cannot open file");
    BufReader::new(file).lines()
}

fn main() {
    println!("Solving problems...");
    let lines = get_data("input.txt");
    let gemma_rate_str = calculate_gemma_rate(lines);
    println!("gemma: {}", gemma_rate_str);
    /*
    let epsilon_rate_str = calculate_epsilon_rate(gemma_rate);

    let gemma_rate = from_binary_str_to_decimal(gemma_rate_str);
    let epsilon_rate = from_binary_str_to_decimal(epsilon_rate_str);

    println!(
        "Gemma rate: {}. Epsilon rate: {}. Power of rates: {}",
        gemma_rate,
        epsilon_rate,
        gemma_rate * epsilon_rate
    );
    */
}
