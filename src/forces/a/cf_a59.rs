// Title: Word
// URL: https://codeforces.com/problemset/problem/59/A

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
        let word = self.get_input().unwrap();

        let (lc, uc) = word.chars().fold((0, 0), |(mut lc, mut uc), c| {
            if c.is_lowercase() {
                lc += 1;
            } else {
                uc += 1;
            }
            (lc, uc)
        });

        let ans = if lc >= uc {
            word.to_lowercase()
        } else {
            word.to_uppercase()
        };

        self.print(ans);
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
        let input = "HoUse";
        let output = "house\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "ViP";
        let output = "VIP\n";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "maTRIx";
        let output = "matrix\n";

        run_solve(input, output);
    }
}
