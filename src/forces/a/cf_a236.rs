// Title: Boy or Girl
// URL: https://codeforces.com/problemset/problem/236/A

use std::collections::HashSet;
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
        let char_count = self
            .get_input()
            .unwrap()
            .chars()
            .collect::<HashSet<char>>()
            .len();

        if char_count % 2 == 0 {
            self.print("CHAT WITH HER!");
        } else {
            self.print("IGNORE HIM!");
        }
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
        let input = "wjmzbmr";
        let output = "CHAT WITH HER!\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "xiaodao";
        let output = "IGNORE HIM!\n";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "sevenkplus";
        let output = "CHAT WITH HER!\n";

        run_solve(input, output);
    }
}
