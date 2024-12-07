// Title: Day 3: Mull It Over
// URL: https://adventofcode.com/2024/day/3#part2

use std::io::prelude::*;
use std::io::{self, BufReader};

use regex::Regex;

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
        // stolen from: https://www.reddit.com/r/adventofcode/comments/1h5frsp/comment/m0equ1o/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();
        let mut sum: u32 = 0;
        let mut enabled: bool = true;

        while let Some(input) = self.get_input() {
            for (full_match, [grp_1, grp_2]) in
                re.captures_iter(input.as_str()).map(|v| v.extract())
            {
                match full_match {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ => {
                        if enabled {
                            sum += grp_1.parse::<u32>().unwrap() * grp_2.parse::<u32>().unwrap()
                        }
                    }
                }
            }
        }

        self.print(sum);
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let output = "48";

        run_solve(input, output);
    }
}
