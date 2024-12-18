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
        // let chars_val = HashMap::from([('a', 1), ('b', 2), ('c', 3), ('d', 4)]);
        let t: u8 = self.get_input().unwrap().parse().unwrap();

        for _ in 0..t {
            // Converting all the inputs to range between 1-4
            let positions = self
                .get_input()
                .unwrap()
                .chars()
                .map(|c| {
                    let val = c.to_digit(18).unwrap() as i8;
                    if val > 13 {
                        // e, f, g, h
                        18 - val
                    } else if val > 9 {
                        // a, b, c, d
                        val - 9
                    } else if val > 4 {
                        // 5 , 6, 7, 8
                        9 - val
                    } else {
                        // 1, 2, 3, 4
                        val
                    }
                })
                .collect::<Vec<i8>>();

            let a = positions[0];
            let b = positions[1];

            let ans = a
                + b
                + match (a, b) {
                    (4, 1) | (1, 4) => -1,
                    (3, 3) => 2,
                    (3, _) | (_, 3) => {
                        if a == 1 || b == 1 {
                            0
                        } else {
                            1
                        }
                    }

                    _ => 0,
                };

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

        assert_eq!(res, output);
    }

    #[test]
    fn test_1() {
        let input = "3\na1\nd4\ng6";
        let output = "2\n8\n6\n";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "10
d3
a8
h8
b2
a6
g5
e6
a1
f1
c3";

        let output = "8
2
2
4
4
6
8
2
4
8
";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "1\na3";
        let output = "4\n";
        run_solve(input, output);
    }
}
