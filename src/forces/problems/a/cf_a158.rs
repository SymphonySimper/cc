// Title: Next Round
// URL: https://codeforces.com/problemset/problem/158/A

use std::io::prelude::*;
use std::io::{self, BufReader};

pub struct Solution<'a> {
    input: BufReader<&'a mut dyn Read>,
    output: &'a mut dyn Write,
}

impl<'a> Solution<'a> {
    pub fn new(input: &'a mut dyn Read, output: &'a mut dyn Write) -> Self {
        Self {
            input: BufReader::new(input),
            output,
        }
    }

    pub fn get_input(&mut self) -> Option<String> {
        let mut input = String::new();
        if let Ok(eof) = self.input.read_line(&mut input) {
            if eof == 0 {
                return None;
            }

            Some(input.trim().to_string())
        } else {
            None
        }
    }

    pub fn print<T: std::fmt::Display>(&mut self, text: T) {
        writeln!(self.output, "{}", text).expect("Valid output");
    }
}

impl<'a> Solution<'a> {
    pub fn solve(&mut self) {
        let k: usize = match self
            .get_input()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..]
        {
            [_, b] => b,
            _ => panic!("Invalid input"),
        };

        let scores: Vec<i8> = self
            .get_input()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();

        let mut count: i8 = 0;
        let target_score = scores[k - 1];
        for score in scores {
            if score >= target_score && score.is_positive() {
                count += 1;
            } else {
                break;
            }
        }

        self.print(count);
    }
}

#[allow(dead_code)]
fn main() {
    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut solution = Solution::new(&mut input, &mut output);
    solution.solve();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_solve(input: &str, output: &str) {
        let mut cursor = io::Cursor::new(input);
        let mut buf: Vec<u8> = Vec::new();

        let mut solution = Solution::new(&mut cursor, &mut buf);
        solution.solve();

        let res = String::from_utf8(buf).expect("valid string");

        assert_eq!(res, output);
    }

    #[test]
    fn test_1() {
        let input = "8 5
10 9 8 7 7 7 5 5";
        let output = "6\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "4 2
0 0 0 0";
        let output = "0\n";

        run_solve(input, output);
    }
}
