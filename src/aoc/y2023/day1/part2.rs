// Title: Day 1: Trebuchet?!
// URL: https://adventofcode.com/2023/day/1#part2

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
    fn get_digit(&self, text: String) -> u32 {
        let nums_map = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut digits: Vec<char> = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut new_text: String = String::new();

        for i in 0..chars.len() {
            if chars[i].is_ascii_digit() {
                new_text.push(chars[i])
            } else {
                for (k, v) in nums_map.iter().enumerate() {
                    if chars[i..].iter().collect::<String>().starts_with(v) {
                        new_text.push_str((k + 1).to_string().as_str());
                    }
                }
            }
        }

        for c in new_text.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            }
        }

        format!("{}{}", digits[0], digits.last().unwrap())
            .parse()
            .unwrap()
    }

    pub fn solve(&mut self) {
        let mut sum: u32 = 0;

        while let Some(input) = self.get_input() {
            sum += Self::get_digit(self, input)
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let output = "281";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "fivethreeonezblqnsfk1
two74119onebtqgnine
jrjh5vsrxbhsfour3
tn5eightfncnzcdtthree8
kpmrk5flx
fkxxqxdfsixgthreepvzjxrkcfk6twofour
dqbx6six5twoone
glmsckj2bvmts1spctnjrtqhmbxzq
7sixthreerzmpbffcx
zhss9gfxfgmrmzthreefivevpkljfourtwoeight
6tfzvrbkfour
sevenfive66five851
fnhksixfour1six81
vkkxbgcqzqflgsvgkkkpfp9five58stsix
eight26sixsghd
7zvrjkcrrgbsix";
        let output = "1017";

        run_solve(input, output);
    }
}
