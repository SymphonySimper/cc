// Title: Stones on the Table
// URL: https://codeforces.com/problemset/problem/266/A

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
        let n = self.get_input().unwrap().parse::<usize>().unwrap();
        let sequence = self.get_input().unwrap().chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = 1;
        let mut count = 0;
        while i < n && j < n {
            while j < n && sequence[i] == sequence[j] {
                j += 1;
                count += 1;
            }
            i = j;
            j = i + 1;
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
        let input = "3\nRRG";
        let output = "1\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "5\nRRRRR";
        let output = "4\n";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "4\nBRBG";
        let output = "0\n";

        run_solve(input, output);
    }

    #[test]
    fn test_4() {
        let input = "4\nRBBR";
        let output = "1\n";

        run_solve(input, output);
    }
}
