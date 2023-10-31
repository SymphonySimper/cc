// Title: Tram
// URL: https://codeforces.com/problemset/problem/116/A

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
        let _ = self.get_input();
        let mut max = 0;
        let mut sum = 0;
        while let Some(input) = self.get_input() {
            let nums: Vec<i32> = input
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            sum = sum - nums[0] + nums[1];
            if sum > max {
                max = sum;
            }
        }

        self.print(max)
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
        let input = "4
0 3
2 5
4 2
4 0";
        let output = "6";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "5
0 4
4 6
6 5
5 4
4 0";
        let output = "6";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "10
0 5
1 7
10 8
5 3
0 5
3 3
8 8
0 6
10 1
9 0";
        let output = "18";

        run_solve(input, output);
    }
}
