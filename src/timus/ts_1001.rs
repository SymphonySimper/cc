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
    // pub fn binary_sqrt(target: f64) -> f64 {
    //     let mut start: f64 = 0.0000;
    //     let mut end = target;
    //
    //     // four decimal places
    //     let eps = 1e-6;
    //
    //     while end - start > eps {
    //         let mid = (end + start) / 2.0;
    //
    //         if target > mid.powi(2) {
    //             start = mid;
    //         } else {
    //             end = mid;
    //         }
    //     }
    //
    //     start
    // }
    pub fn solve(&mut self) {
        let mut values: Vec<f64> = Vec::new();
        while let Some(text) = Self::get_input(self) {
            for v in text.split_whitespace() {
                values.push(v.parse::<f64>().unwrap().sqrt());
            }
        }

        for i in (0..values.len()).rev() {
            self.print(format!("{:.4}", values[i]))
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
        let input = "1427  0   

   876652098643267843 
5276538
";
        let output = "2297.0716
936297014.1164
0.0000
37.7757
";
        run_solve(input, output);
    }
}
