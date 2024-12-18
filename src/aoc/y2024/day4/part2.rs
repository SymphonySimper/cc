// Title: Day 4: Ceres Search
// URL: https://adventofcode.com/2024/day/4#part2

use std::convert::TryFrom;
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
    fn possible_directions(
        &self,
        index: i32,
        row_index: i32,
        grid_size: i32,
        row_size: i32,
    ) -> Option<Vec<Vec<(usize, usize)>>> {
        let mut directions: Vec<Vec<(i32, i32)>> = Vec::new();

        let left = index > 0;
        let right = index < row_size - 1;
        let bottom = row_index < grid_size - 1;
        let top = row_index > 0;

        if !(left && right && bottom && top) {
            return None;
        }

        // left diagonal
        directions.push(
            [
                (row_index - 1, index - 1), // top left
                (row_index, index),         // center
                (row_index + 1, index + 1), // bottom right
            ]
            .to_vec(),
        );

        // right diagonal
        directions.push(
            [
                (row_index - 1, index + 1), // top right
                (row_index, index),         // center
                (row_index + 1, index - 1), // bottom left
            ]
            .to_vec(),
        );

        Some(
            directions
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|index_tuple| {
                            (
                                usize::try_from(index_tuple.0).unwrap(),
                                usize::try_from(index_tuple.1).unwrap(),
                            )
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn get_char(&self, grid: &[Vec<char>], row_index: usize, index: usize) -> char {
        grid[row_index][index]
    }

    pub fn solve(&mut self) {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut count: u32 = 0;

        while let Some(input) = self.get_input() {
            grid.push(input.chars().collect());
        }

        let grid_size = grid.len();
        for (row_index, row) in grid.iter().enumerate() {
            let row_size = row.len();
            for (index, c) in row.iter().enumerate() {
                if *c == 'A' {
                    if let Some(directions) = self.possible_directions(
                        index as i32,
                        row_index as i32,
                        grid_size as i32,
                        row_size as i32,
                    ) {
                        let mut is_x: bool = true;
                        for coordinates in directions {
                            let mut word: String = String::new();
                            for cords in coordinates {
                                word.push(self.get_char(&grid, cords.0, cords.1));
                            }

                            if word != "MAS" && word != "SAM" {
                                is_x = false;
                                break;
                            }
                        }

                        if is_x {
                            count += 1;
                        }
                    };
                }
            }
        }

        self.print(count)
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
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let output = "9";

        run_solve(input, output);
    }
}
