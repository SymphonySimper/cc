// Title: A. Way Too Long Words
// URL: https://codeforces.com/problemset/problem/71/A

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
        let t: u8 = self.get_input().unwrap().parse().unwrap();
        for _ in 0..t {
            let word = self.get_input().unwrap();
            let word_len = word.len();
            if word_len > 10 {
                let mut chars = word.chars();
                self.print(format!(
                    "{}{}{}",
                    chars.nth(0).unwrap(),
                    word_len - 2,
                    chars.last().unwrap(),
                ));
            } else {
                self.print(word);
            }
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
        let input = "4
word
localization
internationalization
pneumonoultramicroscopicsilicovolcanoconiosis";
        let output = "word
l10n
i18n
p43s\n";

        run_solve(input, output);
    }
}
