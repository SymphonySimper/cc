// Title: Day 2: Cube Conundrum
// URL: https://adventofcode.com/2023/day/2@#part2

use std::collections::HashMap;
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
        let mut game: HashMap<i32, HashMap<String, i32>> = HashMap::new();

        while let Some(input) = self.get_input() {
            let mut split = input.split(':');
            let game_id = split
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            for round in split.next().unwrap().trim().to_string().split(';') {
                let balls = round.split(',');
                for ball in balls {
                    let mut ball_split = ball.split_whitespace();
                    let count = ball_split.next().unwrap().trim().parse::<i32>().unwrap();
                    let color = ball_split.next().unwrap().trim().to_string();

                    game.entry(game_id)
                        .and_modify(|m| {
                            m.entry(color.clone())
                                .and_modify(|v| {
                                    if *v < count {
                                        *v = count;
                                    }
                                })
                                .or_insert(count);
                        })
                        .or_insert(HashMap::from([(color, count)]));
                }
            }
        }
        let mut sum = 0;
        for round in game.values() {
            sum += round.values().product::<i32>();
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = "2286";

        run_solve(input, output);
    }
}
