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
        if let Ok(t) = self.get_input().unwrap().parse::<usize>() {
            for _ in 0..t {
                let [n, m] = self
                    .get_input()
                    .unwrap()
                    .split_whitespace()
                    .map(|v| v.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()[0..2]
                else {
                    panic!("Expected 2 inputs");
                };

                let difference = n.max(m) - n.min(m);
                let mut x = self.get_input().unwrap();
                let s = self.get_input().unwrap();
                let mut i = 0;

                loop {
                    if x.contains(&s) {
                        self.print(i);
                        break;
                    } else if i > difference || x.len() > 25 {
                        self.print(-1);
                        break;
                    }

                    x = format!("{x}{x}");

                    i += 1;
                }
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
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test() {
        let name_only = Path::new(file!())
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap();
        let mut f = File::open(format!("src/forces/problems/testcase/{name_only}.txt"))
            .expect("correct test");
        let mut buf: Vec<u8> = Vec::new();

        let mut solution = Solution::new(&mut f, &mut buf);
        solution.solve();

        let res = String::from_utf8(buf).expect("valid string");

        // TODO: Add testcase output
        assert_eq!(
            res,
            "3
1
2
-1
1
0
1
3
1
0
2
5
"
        );
    }
}
