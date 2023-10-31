// Title: Simple Design
// URL: https://codeforces.com/contest/1884/problem/A

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
        while let Some(input) = self.get_input() {
            let (mut a, b): (i32, i32) = match input
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()[..]
            {
                [a, b] => (a, b),
                _ => panic!("Invalid input"),
            };

            let initial_a = a;
            let b_len: i32 = b.to_string().len() as i32;

            let ans;
            loop {
                let a_sum: i32 = a
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().parse::<i32>().unwrap())
                    .sum();
                let a_diff = a_sum - b_len;
                let a_b_diff = a - b_len;
                if a_sum % b == 0 {
                    ans = a;
                    break;
                } else if a_diff % b == 0 && a_b_diff >= initial_a {
                    let a_b_diff_sum = a_b_diff
                        .to_string()
                        .chars()
                        .map(|c| c.to_string().parse::<i32>().unwrap())
                        .sum::<i32>();
                    if a_b_diff_sum % b == 0 {
                        ans = a_b_diff;
                        break;
                    }
                } else {
                    a += b
                }
            }

            self.print(ans);
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

        assert_eq!(res, format!("{output}\n"));
    }

    #[test]
    fn test_1() {
        let input = "6
1 5
10 8
37 9
777 3
1235 10
1 10";
        let output = "5
17
45
777
1243
19";

        run_solve(input, output);
    }
}
