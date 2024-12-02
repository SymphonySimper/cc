// Title: Day 2: Red-Nosed Reports
// URL: https://adventofcode.com/2024/day/2

use std::cmp::Ordering;
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
        let mut safe: u32 = 0;

        'outer: while let Some(input) = self.get_input() {
            let split = input
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let mut direction: &str = "";

            for w in split.windows(2) {
                let a = w[0];
                let b = w[1];
                let sub: u32;

                match a.cmp(&b) {
                    Ordering::Equal => continue 'outer,
                    Ordering::Greater => {
                        if direction == "dec" {
                            continue 'outer;
                        } else {
                            direction = "inc";
                        }
                        sub = a - b;
                    }
                    Ordering::Less => {
                        if direction == "inc" {
                            continue 'outer;
                        } else {
                            direction = "dec";
                        }
                        sub = b - a;
                    }
                }

                if !(sub >= 1 && sub <= 3) {
                    continue 'outer;
                }
            }

            safe += 1;
        }

        self.print(safe)
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
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let output = "2";

        run_solve(input, output);
    }
}
