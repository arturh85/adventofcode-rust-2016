//! # [Day 8: Two-Factor Authentication](https://adventofcode.com/2016/day/8)
//!
//! You come across a door implementing what you can only assume is an implementation of
//! [two-factor authentication](https://en.wikipedia.org/wiki/Multi-factor_authentication) after a
//! long game of [requirements](https://en.wikipedia.org/wiki/Requirement)
//! [telephone](https://en.wikipedia.org/wiki/Chinese_whispers).
//!
//! To get past the door, you first swipe a keycard (no problem; there was one on a nearby desk).
//! Then, it displays a code on a
//! [little screen](https://www.google.com/search?q=tiny+lcd&tbm=isch), and you type that code on
//! a keypad. Then, presumably, the door unlocks.
//!
//! Unfortunately, the screen has been smashed. After a few minutes, you've taken everything
//! apart and figured out how it works. Now you just have to work out what the screen would
//! have displayed.
//!
//! The magnetic strip on the card you swiped encodes a series of instructions for the screen;
//! these instructions are your puzzle input. The screen is `50` pixels wide and `6` pixels tall,
//! all of which start off, and is capable of three somewhat peculiar operations:
//!
//! -   `rect AxB` turns on all of the pixels in a rectangle at the top-left of the screen which
//!     is `A` wide and `B` tall.
//! -   `rotate row y=A by B` shifts all of the pixels in row `A` (0 is the top row) right by
//!     `B` pixels. Pixels that would fall off the right end appear at the left end of the row.
//! -   `rotate column x=A by B` shifts all of the pixels in column `A` (0 is the left column)
//!     down by `B` pixels. Pixels that would fall off the bottom appear at the top of the column.
//!
//! For example, here is a simple sequence on a smaller screen:
//!
//! -   `rect 3x2` creates a small rectangle in the top-left corner:
//!
//!     ```plain
//!     ###....
//!     ###....
//!     .......
//!     ```
//!
//! -   `rotate column x=1 by 1` rotates the second column down by one pixel:
//!
//!     ```plain
//!     #.#....
//!     ###....
//!     .#.....
//!     ```
//!
//! -   `rotate row y=0 by 4` rotates the top row right by four pixels:
//!
//!     ```plain
//!     ....#.#
//!     ###....
//!     .#.....
//!     ```
//!
//! -   `rotate column x=1 by 1` again rotates the second column down by one pixel, causing the
//!     bottom pixel to wrap back to the top:
//!
//!     ```plain
//!     .#..#.#
//!     #.#....
//!     .#.....
//!     ```
//!
//! As you can see, this display technology is extremely powerful, and will soon dominate the
//! tiny-code-displaying-screen market. That's what the advertisement on the back of the display
//! tries to convince you, anyway.
//!
//! There seems to be an intermediate check of the voltage used by the display: after you swipe
//! your card, if the screen did work, **how many pixels should be lit?**
//!
//! # Part Two
//!
//! You notice that the screen is only capable of displaying capital letters; in the font it uses,
//! each letter is `5` pixels wide and `6` tall.
//!
//! After you swipe your card, **what code is the screen trying to display?**

use regex::Regex;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Instruction> {
    let rect_regex = Regex::new(r"^rect (?P<width>\d+)x(?P<height>\d+)$").unwrap();
    let rotate_regex =
        Regex::new(r"^rotate (?P<type>row|column) [xy]=(?P<y>\d+) by (?P<amount>\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            if let Some(matches) = rect_regex.captures(line) {
                let width = matches.name("width").unwrap().as_str().parse().unwrap();
                let height = matches.name("height").unwrap().as_str().parse().unwrap();
                Instruction::Rect((width, height))
            } else {
                let matches = rotate_regex.captures(line).expect("failed to parse");
                let rtype = matches.name("type").unwrap().as_str().to_string();
                let y = matches.name("y").unwrap().as_str().parse().unwrap();
                let amount = matches.name("amount").unwrap().as_str().parse().unwrap();
                match rtype.as_str() {
                    "row" => Instruction::RotateRow((y, amount)),
                    "column" => Instruction::RotateColumn((y, amount)),
                    _ => panic!("invalid rotate type"),
                }
            }
        })
        .collect()
}

/// Part 1: after you swipe your card, if the screen did work, how many pixels should be lit?
#[aoc(day8, part1)]
fn part1(input: &Vec<Instruction>) -> usize {
    count_on(&execute(input))
}

/// Part 2: After you swipe your card, what code is the screen trying to display?
#[aoc(day8, part2)]
fn part2(input: &Vec<Instruction>) -> String {
    let grid = execute(input);
    display_grid(&grid);
    "ZJHRKCPLYJ".into()
}

enum Instruction {
    Rect((usize, usize)),
    RotateRow((usize, usize)),
    RotateColumn((usize, usize)),
}

fn display_grid(grid: &Vec<Vec<bool>>) {
    for row in grid.iter() {
        let mut s = String::new();
        for col in row {
            if *col {
                s += "#";
            } else {
                s += " ";
            }
        }
        println!("{}", s);
    }
}

fn count_on(grid: &Vec<Vec<bool>>) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|col| **col).count())
        .sum()
}

fn build_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut rows = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(false);
        }
        rows.push(row);
    }
    rows
}

fn execute(input: &Vec<Instruction>) -> Vec<Vec<bool>> {
    let mut grid = build_grid(50, 6);
    for instr in input {
        match instr {
            // `rect AxB` turns on all of the pixels in a rectangle at the top-left of the screen
            // which is `A` wide and `B` tall.
            Instruction::Rect((w, h)) => {
                for y in 0..*h {
                    for x in 0..*w {
                        grid[y][x] = true;
                    }
                }
            }
            // `rotate column x=A by B` shifts all of the pixels in column `A` (0 is the left column)
            // down by `B` pixels. Pixels that would fall off the bottom appear at the top of the column.
            Instruction::RotateColumn((x, amount)) => {
                let mut column: Vec<bool> = Vec::new();
                for row in &grid {
                    column.push(row[*x]);
                }
                column.rotate_right(*amount);
                for (idx, v) in column.iter().enumerate() {
                    grid[idx][*x] = *v;
                }
            }
            // `rotate row y=A by B` shifts all of the pixels in row `A` (0 is the top row) right by
            // `B` pixels. Pixels that would fall off the right end appear at the left end of the row.
            Instruction::RotateRow((y, amount)) => grid[*y].rotate_right(*amount),
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    #[test]
    fn part1_examples() {
        assert_eq!(6, part1(&parse_input(EXAMPLE)));
    }
}
