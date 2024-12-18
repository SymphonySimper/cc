// Title: Soldier and Bananas
// URL: https://codeforces.com/problemset/problem/546/A

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
        let (k, n, w): (i32, i32, i32) = match self
            .get_input()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[..]
        {
            [a, b, c] => (a, b, c),
            _ => panic!("Invalid input"),
        };

        let price_of_n_bananas = k * (w * (w + 1) / 2);
        if n >= price_of_n_bananas {
            self.print(0);
        } else {
            self.print(price_of_n_bananas - n);
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
        let input = "3 17 4";
        let output = "13\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "1 2 1";
        let output = "0\n";

        run_solve(input, output);
    }
}
