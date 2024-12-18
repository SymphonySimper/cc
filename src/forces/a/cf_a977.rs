// Title: Wrong Subtraction
// URL: https://codeforces.com/problemset/problem/977/A

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
        let (mut n, k): (i32, u8) = match self
            .get_input()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            [a, b] => (a.parse().unwrap(), b.parse().unwrap()),
            _ => panic!("Invalid input"),
        };

        for _ in 0..k {
            if n % 10 == 0 {
                n /= 10;
            } else {
                n -= 1;
            }
        }

        self.print(n);
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
        let input = "512 4";
        let output = "50\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "1000000000 9";
        let output = "1\n";

        run_solve(input, output);
    }
}
