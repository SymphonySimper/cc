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
        let mut values: Vec<i32> = Vec::with_capacity(3);
        while let Some(n) = self.get_input() {
            values.push(n.parse().unwrap());
        }

        let one_or_zero = |v: i32| v == 1 || v == 0;

        match values[..] {
            [a, b, c] => {
                if one_or_zero(b) || one_or_zero(c) {
                    self.print(a - b - c);
                } else {
                    self.print(a - b * c);
                }
            }
            _ => panic!("Invalid input"),
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

        assert_eq!(res, output);
    }

    #[test]
    fn test_1() {
        let input = "1\n2\n3";
        let output = "-5\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "1\n1\n2";
        let output = "-2\n";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "0\n0\n5";
        let output = "-5\n";

        run_solve(input, output);
    }
}
