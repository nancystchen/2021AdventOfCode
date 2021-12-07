use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct Board {
    // some data structure to hold i/j and value
    // some data structure to hold which numbers are marked
    // to be fast, we need to
    // - mark in constant time (value -> i/j)
    // - check bingo in constant time (check if i or j are a line)
    value_to_position_map: HashMap<u32, (usize, usize)>,
    values: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
    marked_lines: Vec<usize>,
}

impl Board {
    fn new(board_values: Vec<Vec<u32>>) -> Self {
        let value_to_position_map =
            board_values
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (i, row)| {
                    row.iter().enumerate().for_each(|(j, value)| {
                        acc.insert(*value, (i, j));
                    });
                    acc
                });
        Self {
            value_to_position_map,
            marked: (0..board_values.len())
                .map(|_| (0..board_values.len()).map(|_| false).collect())
                .collect(),
            marked_lines: (0..board_values.len() * 2).map(|_| 0).collect(),
            values: board_values,
        }
    }

    // mark a number if possible
    fn mark(&mut self, value: u32) -> Option<(usize, usize)> {
        if let Some(&(i, j)) = self.value_to_position_map.get(&value) {
            self.marked[i][j] = true;
            self.marked_lines[i] += 1;
            self.marked_lines[self.marked.len() + j] += 1;
            Some((i, j))

        } else {
            None
        }
    }

    // check for numbers marked if we have formed a line
    fn check_bingo(&self, pos: &(usize, usize)) -> bool {
        self.marked_lines[pos.0] == 5 || self.marked_lines[self.marked.len() + pos.1] == 5
    }

    fn calculate_winning_score(&self, winning_number: u32) -> u32 {
        let sum = self.marked.iter().enumerate().fold(0, |acc, (i, row)| {
            let row_sum: u32 = row
                .iter()
                .enumerate()
                .map(|(j, makred)| if !makred { self.values[i][j] } else { 0 })
                .sum();
            acc + row_sum
        });
        sum * winning_number
    }
}

fn get_numbers(data: &mut Lines<BufReader<File>>) -> Vec<u32> {
    let numbers_string: String = data.next().unwrap().unwrap();
    numbers_string
        .split(',')
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect()
}

fn create_board(rows: std::iter::Take<&mut Lines<BufReader<File>>>) -> Board {
    let board_vec = rows
        .map(|row_str| {
            let row = row_str.unwrap();
            row.replace("  ", " ")
                .trim_start()
                .split(' ')
                .map(|val| val.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    Board::new(board_vec)
}

fn get_boards(data: &mut Lines<BufReader<File>>) -> Vec<Board> {
    let mut boards = vec![];
    while let Some(line) = data.next() {
        if line.unwrap() == "" {
            let board = create_board(data.take(5));
            boards.push(board);
        }
    }
    boards
}

fn main() {
    println!("Solving problem...");
    let file = File::open("input.txt").unwrap();
    let mut data = BufReader::new(file).lines();
    let numbers = get_numbers(&mut data);
    let mut boards = get_boards(&mut data);
    let mut winning_boards = (0..boards.len()).map(|_| false).collect::<Vec<bool>>();
    let mut num_winning_boards = boards.len();
    let mut winning_board = 0;
    let mut winning_number = 0;

    for n in numbers {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            if let Some(pos) = board.mark(n) {
                let is_bingo = board.check_bingo(&pos);
                if is_bingo && !winning_boards[board_idx] {
                    winning_boards[board_idx] = true; // marked it as won
                    num_winning_boards -= 1;
                    winning_number = n;
                    winning_board = board_idx;
                }
                if num_winning_boards == 0 {
                    break;
                }
            }
        }
        if num_winning_boards == 0 {
            break;
        }
    }
    println!(
        "Winning number: {}. Winning board is {} : {:?}.",
        winning_number, winning_board, boards[winning_board].marked_lines
    );
    let score = boards[winning_board].calculate_winning_score(winning_number);
    println!("Board score is {:?}", score);
}
