// Part 1:
// To find corrupted lines, we
// - try to remove the first open-close pair in string.
// - if the open-close pair's types don't match, then return it.
//
// This should not be confused with incomplete lines, in which
// - when removing open-close paris, we eventually encounter
// - open-open pair. When this happens, skip line.
//
// We can take adventage of Stack.
//

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct ParseError(char);

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse char {} into a brakcet!", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Bracket {
    RoundOpen,
    RoundClosed,
    SquareOpen,
    SquareClosed,
    CurlyOpen,
    CurlyClosed,
    PointyOpen,
    PointyClosed,
}

impl Bracket {
    fn new(ch: &char) -> Result<Self, ParseError> {
        match ch {
            '(' => Ok(Self::RoundOpen),
            ')' => Ok(Self::RoundClosed),
            '[' => Ok(Self::SquareOpen),
            ']' => Ok(Self::SquareClosed),
            '{' => Ok(Self::CurlyOpen),
            '}' => Ok(Self::CurlyClosed),
            '<' => Ok(Self::PointyOpen),
            '>' => Ok(Self::PointyClosed),
            _ => Err(ParseError(ch.to_owned())),
        }
    }

    fn is_open(&self) -> bool {
        matches!(
            self,
            Self::RoundOpen | Self::SquareOpen | Self::CurlyOpen | Self::PointyOpen
        )
    }

    fn is_matching_pair(front: &Self, back: &Self) -> bool {
        match front {
            Self::RoundOpen => back == &Self::RoundClosed,
            Self::SquareOpen => back == &Self::SquareClosed,
            Self::CurlyOpen => back == &Self::CurlyClosed,
            Self::PointyOpen => back == &Self::PointyClosed,
            _ => false,
        }
    }

    fn get_corrupt_points(&self) -> usize {
        match self {
            Self::RoundClosed => 3,
            Self::SquareClosed => 57,
            Self::CurlyClosed => 1197,
            Self::PointyClosed => 25137,
            _ => 0,
        }
    }

    fn get_completion_points(&self) -> usize {
        match self {
            Self::RoundClosed => 1,
            Self::SquareClosed => 2,
            Self::CurlyClosed => 3,
            Self::PointyClosed => 4,
            _ => 0,
        }
    }

    fn turn_open_bracket_to_closed(&self) -> Result<Self, ParseError> {
        match self {
            Self::RoundOpen => Ok(Self::RoundClosed),
            Self::SquareOpen => Ok(Self::SquareClosed),
            Self::CurlyOpen => Ok(Self::CurlyClosed),
            Self::PointyOpen => Ok(Self::PointyClosed),
            _ => Err(ParseError('a')),
        }
    }

    fn find_complete_sequence(openings: Vec<Bracket>) -> Vec<Bracket> {
        openings
            .into_iter()
            .map(|b| Bracket::turn_open_bracket_to_closed(&b).unwrap())
            .rev()
            .collect()
    }
}

fn calculate_completion_score(brackets_list: Vec<Vec<Bracket>>) -> usize {
    let mut scores = brackets_list.iter()
        .map(|brackets| {
            brackets
                .iter()
                .fold(0, |acc, b| acc * 5 + b.get_completion_points())
        })
        .collect::<Vec<usize>>();
    scores.sort_unstable();
    let middle_idx = scores.len() / 2;
    scores[middle_idx]
}

fn calculate_corrupt_score(bugs: Vec<Bracket>) -> usize {
    bugs.iter().map(|b| b.get_corrupt_points()).sum()
}

// Given a vector of brackets, find the sequence of brackets to complete line.
fn find_incomplete_brackets(list: &Vec<Bracket>) -> Option<Vec<Bracket>> {
    let mut stack: Vec<Bracket> = vec![];
    for b in list {
        if b.is_open() {
            stack.push(b.clone());
        } else if let Some(front) = stack.last() {
            if Bracket::is_matching_pair(front, b) {
                stack.pop();
            } else {
                return None;
            }
        }
    }
    Some(Bracket::find_complete_sequence(stack))
}

// Given a vector of brackets, find the first corrupted bracket.
fn find_corrupted_brackets(list: &Vec<Bracket>) -> Option<Bracket> {
    let mut stack: Vec<Bracket> = vec![];
    for b in list {
        if b.is_open() {
            stack.push(b.clone());
        } else if let Some(front) = stack.last() {
            if Bracket::is_matching_pair(front, b) {
                stack.pop();
            } else {
                return Some(b.clone());
            }
        }
    }
    None
}

// Given a file buffer, for each line in it, generate a linked list of bracket.
fn parse_data(data: Lines<BufReader<File>>) -> Vec<Vec<Bracket>> {
    data.map(|line| {
        if let Ok(string) = line {
            string
                .chars()
                .map(|ch| Bracket::new(&ch).unwrap())
                .collect::<Vec<Bracket>>()
        } else {
            Vec::new()
        }
    })
    .collect()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let data = BufReader::new(file).lines();
    let lists = parse_data(data);
    let corrupted_brackets = lists
        .iter()
        .filter_map(find_corrupted_brackets)
        .collect::<Vec<Bracket>>();
    let total_score = calculate_corrupt_score(corrupted_brackets);
    println!("Total corruption score: {}", total_score);
    let seqs = lists
        .iter()
        .filter_map(find_incomplete_brackets)
        .collect::<Vec<Vec<Bracket>>>();
    let completion_score = calculate_completion_score(seqs);
    println!("Total completion score: {}", completion_score);
}
