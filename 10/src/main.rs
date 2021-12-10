// Part 1:
// To find corrupted lines, we
// - try to remove the first open-close pair in string.
// - if the open-close pair's types don't match, then return it.
//
// This should not be confused with incomplete lines, in which
// - when removing open-close paris, we eventually encounter
// - open-open pair. When this happens, skip line.
//
// We can take adventage of LinkedList so we don't keep resizing
// a vector! Appending two parts should take O(1), so overall
// operation is O(n).
//
//

use std::collections::LinkedList;
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

#[derive(Debug, PartialEq)]
enum Brackets {
    RoundOpen,
    RoundClosed,
    SquareOpen,
    SquareClosed,
    CurlyOpen,
    CurlyClosed,
    PointyOpen,
    PointyClosed,
}

impl Brackets {
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

    fn is_closed(&self) -> bool {
        matches!(
            self,
            Self::RoundClosed | Self::SquareClosed | Self::CurlyClosed | Self::PointyClosed
        )
    }

    fn is_open_close_pair(front: &Self, back: &Self) -> bool {
        front.is_open() && back.is_closed()
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

    fn get_points(&self) -> usize {
        match self {
            Self::RoundClosed => 3,
            Self::SquareClosed => 57,
            Self::CurlyClosed => 1197,
            Self::PointyClosed => 25137,
            _ => 0,
        }
    }
}

fn calculate_corrupt_score(bugs: Vec<char>) -> usize {
    unimplemented!()
}

// Given a linked list of chars, find the corrupted bracket if it exists.
fn find_corrupted_bracket(list: LinkedList<char>) -> Option<char> {
    unimplemented!()
}

// Given a file buffer, for each line in it, generate a linked list of chars.
fn parse_data(data: Lines<BufReader<File>>) -> Vec<LinkedList<char>> {
    unimplemented!()
}

fn main() {
    let file = File::open("sample_input.txt").unwrap();
    let data = BufReader::new(file).lines();
    let lists = parse_data(data);
    let corrupted_chars = lists
        .into_iter()
        .filter_map(find_corrupted_bracket)
        .collect::<Vec<char>>();
}
