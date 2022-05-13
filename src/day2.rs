//! # [Day 2: Bathroom Security](https://adventofcode.com/2016/day/2)
//!
//! You arrive at Easter Bunny Headquarters under cover of darkness. However, you left in such a
//! rush that you forgot to use the bathroom! Fancy office buildings like this one usually have
//! keypad locks on their bathrooms, so you search the front desk for the code.
//!
//! "In order to improve security," the document you find says, "bathroom codes will no longer be
//! written down. Instead, please memorize and follow the procedure below to access the bathrooms."
//!
//! The document goes on to explain that each button to be pressed can be found by starting on the
//! previous button and moving to adjacent buttons on the keypad: `U` moves up, `D` moves down, `L`
//! moves left, and `R` moves right. Each line of instructions corresponds to one button, starting
//! at the previous button (or, for the first line, the "5" button); press whatever button you're
//! on at the end of each line. If a move doesn't lead to a button, ignore it.
//!
//! You can't hold it much longer, so you decide to figure out the code as you walk to the bathroom.
//! You picture a keypad like this:
//!
//! ```plain
//! 1 2 3
//! 4 5 6
//! 7 8 9
//! ```
//!
//! Suppose your instructions are:
//!
//! ```plain
//! ULL
//! RRDDD
//! LURDL
//! UUUUD
//! ```
//!
//! -   You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"),
//!     so the first button is `1`.
//! -   Starting from the previous button ("1"), you move right twice (to "3") and then down three
//!     times (stopping at "9" after two moves and ignoring the third), ending up with `9`.
//! -   Continuing from "9", you move left, up, right, down, and left, ending with `8`.
//! -   Finally, you move up four times (stopping at "2"), then down once, ending with `5`.
//!
//! So, in this example, the bathroom code is `1985`.
//!
//! Your puzzle input is the instructions from the document you found at the front desk.
//!
//! **What is the bathroom code?**
//!
//! # Part Two
//!
//! You finally arrive at the bathroom (it's a several minute walk from the lobby so visitors can
//! behold the many fancy conference rooms and water coolers on this floor) and go to punch in
//! the code. Much to your bladder's dismay, the keypad is not at all like you imagined it.
//! Instead, you are confronted with the result of hundreds of man-hours of
//! bathroom-keypad-design meetings:
//!
//! ```plain
//!     1
//!   2 3 4
//! 5 6 7 8 9
//!   A B C
//!     D
//! ```
//!
//! You still start at "5" and stop when you're at an edge, but given the same
//! instructions as above, the outcome is very different:
//!
//! -   You start at "5" and don't move at all (up and left are both edges), ending at `5`.
//! -   Continuing from "5", you move right twice and down three times
//!     (through "6", "7", "B", "D", "D"), ending at `D`.
//! -   Then, from "D", you move five more times (through "D", "B", "C", "C", "B"), ending at `B`.
//! -   Finally, after five more moves, you end at `3`.
//!
//! So, given the actual keypad layout, the code would be `5DB3`.
//!
//! Using the same instructions in your puzzle input, what is the correct bathroom code?

const KEYPAD_1: &str = "123
456
789";

const KEYPAD_2: &str = "  1  
 234 
56789
 ABC 
  D  ";

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Vec<Instr>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'L' => Instr::Left,
                    'R' => Instr::Right,
                    'U' => Instr::Up,
                    'D' => Instr::Down,
                    _ => panic!("invalid input"),
                })
                .collect()
        })
        .collect()
}

/// Part 1: What is the bathroom code?
#[aoc(day2, part1)]
fn part1(input: &Vec<Vec<Instr>>) -> String {
    execute(input, KEYPAD_1, '5')
}

/// Part 2: Using the same instructions in your puzzle input, what is the correct bathroom code?
#[aoc(day2, part2)]
fn part2(input: &Vec<Vec<Instr>>) -> String {
    execute(input, KEYPAD_2, '5')
}

#[derive(Debug, Clone)]
enum Instr {
    Up,
    Down,
    Left,
    Right,
}

impl Instr {
    fn vec(&self) -> (i64, i64) {
        match self {
            Instr::Up => (-1, 0),
            Instr::Down => (1, 0),
            Instr::Left => (0, -1),
            Instr::Right => (0, 1),
        }
    }
}

fn keypad_get(keypad: &str, pos: (i64, i64)) -> Option<char> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }
    let line = keypad.lines().nth(pos.0 as usize)?;
    let key = line.chars().nth(pos.1 as usize)?;
    if key != ' ' {
        Some(key)
    } else {
        None
    }
}

fn execute(input: &Vec<Vec<Instr>>, keypad: &str, start: char) -> String {
    let mut code = String::new();
    let mut pos = (0, 0);
    for (row, line) in keypad.lines().enumerate() {
        if let Some(col) = line.find(start) {
            pos.0 = row as i64;
            pos.1 = col as i64;
            break;
        }
    }
    for line in input {
        for instr in line {
            let vec = instr.vec();
            let target = (pos.0 + vec.0, pos.1 + vec.1);
            if let Some(_) = keypad_get(keypad, target) {
                pos = target;
            }
        }
        code += &keypad_get(keypad, pos).unwrap().to_string();
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn part1_examples() {
        let input = parse_input(EXAMPLE);
        // So, in this example, the bathroom code is `1985`.
        assert_eq!(execute(&input, KEYPAD_1, '5'), "1985");
    }

    #[test]
    fn part2_examples() {
        let input = parse_input(EXAMPLE);
        // So, given the actual keypad layout, the code would be `5DB3`.
        assert_eq!(execute(&input, KEYPAD_2, '5'), "5DB3");
    }
}
