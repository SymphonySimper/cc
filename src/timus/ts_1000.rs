use std::fmt::Debug;
#[allow(unused_imports)]
use std::io::{self, BufReader};
use std::io::{prelude::*, Lines};
use std::str::FromStr;

pub struct Solution<'a> {
    lines: Lines<BufReader<&'a mut dyn Read>>,
    output: &'a mut dyn Write,
}

impl<'a> Solution<'a> {
    pub fn new(input: &'a mut dyn Read, output: &'a mut dyn Write) -> Self {
        Self {
            lines: BufReader::new(input).lines(),
            output,
        }
    }
    pub fn get_input(&mut self) -> String {
        self.lines.next().unwrap().unwrap().trim().to_string()
    }

    pub fn parse<T: FromStr>(input: impl Into<String>) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let input = input.into();
        input.parse::<T>().unwrap()
    }

    pub fn split_and_parse<T: FromStr>(&mut self, input: Option<String>) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let input = if let Some(text) = input {
            text
        } else {
            Self::get_input(self)
        };

        input.split_whitespace().map(|v| Self::parse(v)).collect()
    }

    pub fn print<T: std::fmt::Display>(&mut self, text: T) {
        writeln!(self.output, "{}", text).expect("Valid output");
    }
}

impl<'a> Solution<'a> {
    pub fn solve(&mut self) {
        let sum: i32 = Self::split_and_parse(self, None).iter().sum();
        Self::print(self, sum);
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
        let input = "1 5";
        let output = "6\n";

        run_solve(input, output);
    }
}
