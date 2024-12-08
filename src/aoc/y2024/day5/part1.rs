// Title: Day 5: Print Queue
// URL: https://adventofcode.com/2024/day/5

use std::collections::{HashMap, HashSet};
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

impl Solution<'_> {
    pub fn solve(&mut self) {
        let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut updates: Vec<Vec<u32>> = Vec::new();

        let mut sum: u32 = 0;

        while let Some(input) = self.get_input() {
            if input.is_empty() {
                continue;
            }

            if input.contains("|") {
                let (a, b) = match input
                    .split("|")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()[..]
                {
                    [a, b] => (a, b),
                    _ => panic!("Found only one of rule"),
                };
                rules.entry(a).or_default().insert(b);
            } else {
                updates.push(
                    input
                        .split(",")
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                )
            }
        }

        'update_loop: for update in updates {
            let middle_number = update[update.len() / 2];
            for (index, page) in update.iter().enumerate() {
                if !rules.contains_key(page) || index == 0 {
                    continue;
                }

                if let Some(rule) = rules.get(page) {
                    for before in &update[0..index] {
                        if rule.contains(before) {
                            continue 'update_loop;
                        }
                    }
                }
            }

            sum += middle_number;
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
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let output = "143";

        run_solve(input, output);
    }
}
