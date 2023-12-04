// Title: Day 1: Trebuchet?!
// URL: https://adventofcode.com/2023/day/1

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
        let mut sum: i32 = 0;
        while let Some(input) = self.get_input() {
            let mut digits: Vec<i32> = Vec::new();
            for c in input.chars() {
                if c.is_ascii_digit() {
                    digits.push(c.to_string().parse().unwrap())
                }
            }

            sum += format!("{}{}", digits[0], digits.last().unwrap())
                .parse::<i32>()
                .unwrap();
        }

        self.print(sum)
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let output = "142";

        run_solve(input, output);
    }
}
