// Title: Bear and Big Brother
// URL: https://codeforces.com/problemset/problem/791/A

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
        let (mut l, mut b): (u16, u16) = match self
            .get_input()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()[..]
        {
            [a, b] => (a, b),
            _ => panic!("Invalid input"),
        };

        let mut count = 0;
        loop {
            l *= 3;
            b *= 2;
            count += 1;

            if l > b {
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
        let input = "4 7";
        let output = "2\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "4 9";
        let output = "3\n";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "1 1";
        let output = "1\n";

        run_solve(input, output);
    }
}
