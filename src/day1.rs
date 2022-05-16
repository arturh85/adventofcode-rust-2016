//! # [Day 1: No Time for a Taxicab](https://adventofcode.com/2016/day/1)
//!
//! Santa's sleigh uses a very high-precision clock to guide its movements, and the clock's
//! oscillator is regulated by stars. Unfortunately, the stars have been stolen...
//! by the Easter Bunny. To save Christmas, Santa needs you to retrieve all fifty
//! stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
//! calendar; the second puzzle is unlocked when you complete the first.
//! Each puzzle grants one star. Good luck!
//!
//! You're airdropped near Easter Bunny Headquarters in a city somewhere. "Near", unfortunately,
//! is as close as you can get - the instructions on the Easter Bunny Recruiting Document the Elves
//! intercepted start here, and nobody had time to work them out further.
//!
//! The Document indicates that you should start at the given coordinates (where you just landed)
//! and face North. Then, follow the provided sequence: either turn left (`L`) or right (`R`)
//! 90 degrees, then walk forward the given number of blocks, ending at a new intersection.
//!
//! There's no time to follow such ridiculous instructions on foot, though, so you take a moment
//! and work out the destination. Given that you can only walk on the
//! [street grid of the city](https://en.wikipedia.org/wiki/Taxicab_geometry),
//! how far is the shortest path to the destination?
//!
//! For example:
//!
//! -   Following `R2, L3` leaves you `2` blocks East and `3` blocks North, or `5` blocks away.
//! -   `R2, R2, R2` leaves you `2` blocks due South of your starting position, which is `2`
//!      blocks away.
//! -   `R5, L5, R5, R3` leaves you `12` blocks away.
//!
//! **How many blocks away is Easter Bunny HQ?**
//!
//! # Part Two
//!
//! Then, you notice the instructions continue on the back of the Recruiting Document.
//! Easter Bunny HQ is actually at the first location you visit twice.
//!
//! For example, if your instructions are R8, R4, R4, R8, the first location you visit twice is 4
//! blocks away, due East.
//!
//! How many blocks away is the first location you visit twice?

use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Instr> {
    input
        .split(", ")
        .map(|part| match part.chars().next().unwrap() {
            'L' => Instr::Left(part[1..].parse().unwrap()),
            'R' => Instr::Right(part[1..].parse().unwrap()),
            _ => panic!("invalid input"),
        })
        .collect()
}

/// Part 1: How many blocks away is Easter Bunny HQ?
#[aoc(day1, part1)]
fn part1(input: &Vec<Instr>) -> u64 {
    let pos = execute1(input);
    manhattan_distance(pos)
}

/// Part 2: How many blocks away is the first location you visit twice?
#[aoc(day1, part2)]
fn part2(input: &Vec<Instr>) -> u64 {
    let pos = execute2(input).unwrap();
    manhattan_distance(pos)
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn left(&self) -> Direction {
        let val = ToPrimitive::to_u8(self).unwrap();
        // println!("left {}", val);
        FromPrimitive::from_u8(if val > 0 { val - 1 } else { 3 }).unwrap()
    }
    fn right(&self) -> Direction {
        let val = (ToPrimitive::to_u8(self).unwrap() + 1) % 4;
        // println!("right {}", val);
        FromPrimitive::from_u8(val).unwrap()
    }
    fn vec(&self) -> (i64, i64) {
        match self {
            Direction::North => (1, 0),
            Direction::East => (0, 1),
            Direction::South => (-1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
enum Instr {
    Left(u64),
    Right(u64),
}

fn execute1(instructions: &Vec<Instr>) -> (i64, i64) {
    let mut pos = (0, 0);
    let mut direction = Direction::North;
    for instr in instructions {
        let amount = match instr {
            Instr::Left(amount) => {
                direction = direction.left();
                *amount
            }
            Instr::Right(amount) => {
                direction = direction.right();
                *amount
            }
        };
        let vec = direction.vec();
        pos.0 += vec.0 * amount as i64;
        pos.1 += vec.1 * amount as i64;
    }
    pos
}

fn execute2(instructions: &Vec<Instr>) -> Option<(i64, i64)> {
    let mut pos = (0, 0);
    let mut direction = Direction::North;
    let mut history: Vec<(i64, i64)> = vec![pos];
    for instr in instructions {
        let amount = match instr {
            Instr::Left(amount) => {
                direction = direction.left();
                *amount
            }
            Instr::Right(amount) => {
                direction = direction.right();
                *amount
            }
        };
        let vec = direction.vec();
        for _ in 0..amount {
            pos.0 += vec.0;
            pos.1 += vec.1;
            if history.contains(&pos) {
                return Some(pos);
            }
            history.push(pos);
        }
    }
    None
}

// see https://en.wikipedia.org/wiki/Taxicab_geometry
fn manhattan_distance(pos: (i64, i64)) -> u64 {
    (pos.0.abs() + pos.1.abs()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        // Following `R2, L3` leaves you `2` blocks East and `3` blocks North, or `5` blocks away.
        let example1 = parse_input("R2, L3");
        let target1 = execute1(&example1);
        assert_eq!(target1, (3, 2));
        assert_eq!(manhattan_distance(target1), 5);

        // `R2, R2, R2` leaves you `2` blocks due South of your starting position, which is `2`
        //  blocks away.
        let example2 = parse_input("R2, R2, R2");
        let target2 = execute1(&example2);
        assert_eq!(target2, (-2, 0));
        assert_eq!(manhattan_distance(target2), 2);

        // `R5, L5, R5, R3` leaves you `12` blocks away.
        let example3 = parse_input("R5, L5, R5, R3");
        let target3 = execute1(&example3);
        assert_eq!(manhattan_distance(target3), 12);
    }

    #[test]
    fn part2_examples() {
        // For example, if your instructions are R8, R4, R4, R8, the first location you visit twice is 4 blocks away, due East.
        let example = parse_input("R8, R4, R4, R8");
        let target = execute2(&example).unwrap();
        assert_eq!(target, (0, 4));
        assert_eq!(manhattan_distance(target), 4);
    }
}
