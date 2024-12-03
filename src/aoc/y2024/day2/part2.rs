// Title: Day 2: Red-Nosed Reports
// URL: https://adventofcode.com/2024/day/2#part2

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
    fn check_safe(&self, split: &Vec<u32>) -> Result<(), u32> {
        let mut direction: &str = "";

        let mut safe: bool = true;
        let mut i: u32 = 0;
        for w in split.windows(2) {
            let (a, b) = match w[..] {
                [a, b] => (a, b),
                _ => panic!("Inavlid input"),
            };
            let mut sub: u32 = 0;

            match a.cmp(&b) {
                Ordering::Equal => {
                    safe = false;
                }
                Ordering::Less => {
                    if direction == "inc" {
                        safe = false;
                    } else {
                        direction = "dec";
                        sub = b - a;
                    }
                }
                Ordering::Greater => {
                    if direction == "dec" {
                        safe = false;
                    } else {
                        direction = "inc";
                        sub = a - b;
                    }
                }
            }

            if sub == 0 || sub > 3 {
                safe = false;
            }

            if !safe {
                return Err(i);
            }

            i += 1;
        }

        return Ok(());
    }

    fn gen_split(&self, split: &Vec<u32>, index: u32) -> Vec<u32> {
        return split
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != (index as usize))
            .map(|(_, v)| v.to_owned())
            .collect::<Vec<u32>>();
    }

    pub fn solve(&mut self) {
        let mut safe: u32 = 0;

        while let Some(input) = self.get_input() {
            let split = input
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if let Err(index) = self.check_safe(&split) {
                let i = self.check_safe(&self.gen_split(&split, index)).is_err();
                let i_plus = self.check_safe(&self.gen_split(&split, index + 1)).is_err();
                if index != 0 {
                    let i_minus = self.check_safe(&self.gen_split(&split, index - 1)).is_err();

                    if i && i_plus && i_minus {
                        continue;
                    }
                } else {
                    if i && i_plus {
                        continue;
                    }
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
        let output = "4";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "82 84 85 87 90 92 93 91
7 10 12 14 17 19 22 22
66 68 69 72 74 78
10 11 13 14 17 18 25
34 36 39 41 43 42 44
35 37 40 37 39 38
12 13 10 11 11";
        // safe
        // safe
        // safe
        // safe
        // safe
        // unsafe
        // unsafe
        let output = "5";

        run_solve(input, output);
    }
}
