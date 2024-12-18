// Title: Day 4: Ceres Search
// URL: https://adventofcode.com/2024/day/4

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
    ) -> Vec<Vec<(usize, usize)>> {
        let mut directions: Vec<Vec<(i32, i32)>> = Vec::new();

        let left = index >= 3;
        let right = index + 3 <= row_size - 1;
        let bottom = row_index + 3 <= grid_size - 1;
        let top = row_index - 3 >= 0;

        // left of index
        if left {
            directions.push((0..=3).map(|s| (row_index, index - s)).collect());
        }

        // right of index
        if right {
            directions.push((0..=3).map(|a| (row_index, index + a)).collect());
        }

        // bottom of index
        if bottom {
            directions.push((0..=3).map(|a| (row_index + a, index)).collect());

            // bottom left of index
            if left {
                directions.push((0..=3).map(|s| (row_index + s, index - s)).collect());
            }

            // bottom right of index
            if right {
                directions.push((0..=3).map(|a| (row_index + a, index + a)).collect());
            }
        }

        // top of index
        if top {
            directions.push((0..=3).map(|s| (row_index - s, index)).collect());

            // top left of index
            if left {
                directions.push((0..=3).map(|s| (row_index - s, index - s)).collect());
            }

            // top right of index
            if right {
                directions.push((0..=3).map(|a| (row_index - a, index + a)).collect());
            }
        }

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
            .collect()
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
                if *c == 'X' {
                    let directions = self.possible_directions(
                        index as i32,
                        row_index as i32,
                        grid_size as i32,
                        row_size as i32,
                    );

                    for coordinates in directions {
                        let mut word: String = String::new();
                        for cords in coordinates {
                            word.push(self.get_char(&grid, cords.0, cords.1));
                        }

                        if word == "XMAS" || word == "SAMX" {
                            count += 1;
                        }
                    }
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
        let output = "18";

        run_solve(input, output);
    }
}
