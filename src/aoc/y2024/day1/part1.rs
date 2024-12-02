// Title: Day 1: Historian Hysteria
// URL: https://adventofcode.com/2024/day/1

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
        let mut a: Vec<u32> = Vec::new();
        let mut b: Vec<u32> = Vec::new();

        while let Some(list_values) = self.get_input() {
            let split = list_values
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            a.push(split[0]);
            b.push(split[1]);
        }

        a.sort();
        b.sort();

        let mut sum: u32 = 0;
        for i in 0..a.len() {
            let av = a[i];
            let bv = b[i];

            sum += match av.cmp(&bv) {
                Ordering::Equal | Ordering::Greater => av - bv,
                Ordering::Less => bv - av,
            }
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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let output = "11";

        run_solve(input, output);
    }
}
