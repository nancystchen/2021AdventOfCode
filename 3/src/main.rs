use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn from_binary_str_to_decimal(binary_str: &str) -> i32 {
    binary_str
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, ch)| {
            let bit: u32 = ch.into();
            acc + 2_i32.pow(idx as u32) * ((bit - 48)as i32)
        })
}

fn calculate_epsilon_rate(gemma_rate: &str) -> String {
    String::from_iter(
        gemma_rate
            .chars()
            .map(|ch| if ch == '1' { '0' } else { '1' }),
    )
}

fn calculate_gemma_rate(data: Lines<BufReader<File>>) -> String {
    let mut lines = data.peekable();
    let mut num_one_in_binary_position: Vec<usize> = lines
        .peek()
        .expect("Cannot find the first line of data")
        .as_ref()
        .map(|line| vec![0; line.len()])
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

    num_one_in_binary_position
        .into_iter()
        .fold(String::new(), |mut acc, count| {
            if count > (total_line_count / 2) {
                acc.push('1');
            } else {
                acc.push('0');
            }
            acc
        })
}

fn get_data(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).expect("Cannot open file");
    BufReader::new(file).lines()
}

fn main() {
    println!("Solving problems...");
    let lines = get_data("input.txt");
    let gemma_rate_str = calculate_gemma_rate(lines);
    let epsilon_rate_str = calculate_epsilon_rate(&gemma_rate_str);

    let gemma_rate = from_binary_str_to_decimal(&gemma_rate_str);
    println!("epsilon calc");
    let epsilon_rate = from_binary_str_to_decimal(&epsilon_rate_str);

    println!(
        "Gemma rate: {}. Epsilon rate: {}. Power of rates: {}",
        gemma_rate,
        epsilon_rate,
        gemma_rate * epsilon_rate
    );
}
