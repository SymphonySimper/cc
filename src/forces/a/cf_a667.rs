// Title: Vanya and Fence
// URL: https://codeforces.com/problemset/problem/677/A

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
        let max_height: i32 = self
            .get_input()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let width: i32 = self
            .get_input()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .fold(0, |mut w, h: i32| {
                if h > max_height {
                    w += 2;
                } else {
                    w += 1;
                }
                w
            });

        self.print(width);
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

        assert_eq!(res, format!("{output}\n"));
    }

    #[test]
    fn test_1() {
        let input = "3 7\n4 5 14";
        let output = "4";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "6 1\n1 1 1 1 1 1";
        let output = "6";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "6 5\n7 6 8 9 10 5";
        let output = "11";

        run_solve(input, output);
    }
}
