//! # [Day 10: Balance Bots](https://adventofcode.com/2016/day/10)
//!
//! You come upon a factory in which many robots are
//! [zooming around](https://www.youtube.com/watch?v=JnkMyfQ5YfY&t=40) handing small microchips
//! to each other.
//!
//! Upon closer examination, you notice that each bot only proceeds when it has two microchips,
//! and once it does, it gives each one to a different bot or puts it in a marked "output" bin.
//! Sometimes, bots take microchips from "input" bins, too.
//!
//! Inspecting one of the microchips, it seems like they each contain a single number;
//! the bots must use some logic to decide what to do with each chip. You access the local control
//! computer and download the bots' instructions (your puzzle input).
//!
//! Some of the instructions specify that a specific-valued microchip should be given to a
//! specific bot; the rest of the instructions indicate what a given bot should do with its
//! lower-value or higher-value chip.
//!
//! For example, consider the following instructions:
//!
//! ```plain
//! value 5 goes to bot 2
//! bot 2 gives low to bot 1 and high to bot 0
//! value 3 goes to bot 1
//! bot 1 gives low to output 1 and high to bot 0
//! bot 0 gives low to output 2 and high to output 0
//! value 2 goes to bot 2
//!
//! ```
//!
//! -   Initially, bot `1` starts with a value-`3` chip, and bot `2` starts with a
//!     value-`2` chip and a value-`5` chip.
//! -   Because bot `2` has two microchips, it gives its lower one (`2`) to bot `1` and its
//!     higher one (`5`) to bot `0`.
//! -   Then, bot `1` has two microchips; it puts the value-`2` chip in output `1` and gives
//!     the value-`3` chip to bot `0`.
//! -   Finally, bot `0` has two microchips; it puts the `3` in output `2` and the `5` in
//!     output `0`.
//!
//! In the end, output bin `0` contains a value-`5` microchip, output bin `1` contains a
//! value-`2` microchip, and output bin `2` contains a value-`3` microchip. In this configuration,
//! bot number `2` is responsible for comparing value-`5` microchips with value-`2` microchips.
//!
//! Based on your instructions, **what is the number of the bot that is responsible for comparing
//! value-`61` microchips with value-`17` microchips?**
//!
//! # Part 2
//!
//! **What do you get if you multiply together the values of one chip in each of
//! outputs 0, 1, and 2?**

use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Instruction> {
    let value_re = Regex::new(r"^value (?P<value>\d+) goes to bot (?P<bot>\d+)$").unwrap();
    let bot_re = Regex::new(r"^bot (?P<bot>\d+) gives low to (?P<low_target>bot|output) (?P<low_target_nr>\w+) and high to (?P<high_target>bot|output) (?P<high_target_nr>\w+)$").unwrap();
    input
        .lines()
        .map(|line| {
            if let Some(matches) = value_re.captures(line) {
                let value = matches.name("value").unwrap().as_str().parse().unwrap();
                let bot = matches.name("bot").unwrap().as_str().parse().unwrap();
                Instruction::ValueToBot((bot, value))
            } else {
                let matches = bot_re.captures(line).unwrap();
                let bot = matches.name("bot").unwrap().as_str().parse().unwrap();
                let low_target_nr = matches
                    .name("low_target_nr")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                let low_target = match matches.name("low_target").unwrap().as_str() {
                    "bot" => Target::Bot(low_target_nr),
                    "output" => Target::Output(low_target_nr),
                    _ => panic!("invalid low_target"),
                };
                let high_target_nr = matches
                    .name("high_target_nr")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                let high_target = match matches.name("high_target").unwrap().as_str() {
                    "bot" => Target::Bot(high_target_nr),
                    "output" => Target::Output(high_target_nr),
                    _ => panic!("invalid high_target"),
                };
                Instruction::BotTo((bot, low_target, high_target))
            }
        })
        .collect()
}

/// Part 1:
/// what is the number of the bot that is responsible for comparing value-`61` microchips
/// with value-`17` microchips?
#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> u32 {
    Factory::look_for(input, 17, 61).unwrap()
}

/// Part 2: What do you get if you multiply together the values of one chip in each of
/// outputs 0, 1, and 2?
#[aoc(day10, part2)]
fn part2(input: &Vec<Instruction>) -> u32 {
    let mut state = Factory::new();
    state.exec(input);
    (0..3_u32).map(|n| state.outputs.get(&n).unwrap()).product()
}

struct Factory {
    bots: HashMap<u32, Vec<u32>>,
    queue: HashMap<u32, (Target, Target)>,
    outputs: HashMap<u32, u32>,
    look_for: Option<(u32, u32)>,
    look_target: Option<u32>,
}

impl Factory {
    fn new() -> Self {
        Factory {
            bots: HashMap::new(),
            queue: HashMap::new(),
            outputs: HashMap::new(),
            look_for: None,
            look_target: None,
        }
    }

    fn look_for(input: &[Instruction], low: u32, high: u32) -> Option<u32> {
        let mut state = Factory::new();
        state.look_for = Some((low, high));
        state.exec(input);
        state.look_target
    }

    fn exec(&mut self, instructions: &[Instruction]) {
        for instr in instructions {
            self.exec_single(instr);
        }
    }

    fn exec_single(&mut self, instr: &Instruction) {
        match instr {
            Instruction::ValueToBot((bot_nr, value)) => self.bot_add(*bot_nr, *value),
            Instruction::BotTo((bot_nr, low_target, high_target)) => {
                let bot_values = self.bot_values(*bot_nr);
                if bot_values.len() == 2 {
                    self.give(*bot_nr, low_target, high_target);
                } else {
                    self.queue
                        .insert(*bot_nr, (low_target.clone(), high_target.clone()));
                }
            }
        };
    }

    fn give(&mut self, bot_nr: u32, low_target: &Target, high_target: &Target) {
        let bot_values = self.bot_values(bot_nr);
        if bot_values.len() == 2 {
            let low = bot_values[0];
            let high = bot_values[1];
            if let Some((look_low, look_high)) = self.look_for {
                if low == look_low && high == look_high {
                    self.look_target = Some(bot_nr);
                }
            }
            self.target_add(low_target, low);
            self.target_add(high_target, high);
            self.bots.remove(&bot_nr);
        }
    }

    fn target_add(&mut self, target: &Target, value: u32) {
        match target {
            Target::Bot(bot_nr) => self.bot_add(*bot_nr, value),
            Target::Output(output_nr) => {
                self.outputs.insert(*output_nr, value);
            }
        }
    }

    fn bot_add(&mut self, bot_nr: u32, value: u32) {
        if let Some(values) = self.bots.get_mut(&bot_nr) {
            values.push(value);
            values.sort();
            if values.len() == 2 {
                if let Some((low_target, high_target)) = self.queue.get(&bot_nr) {
                    let low_target = low_target.clone();
                    let high_target = high_target.clone();
                    self.give(bot_nr, &low_target, &high_target);
                    self.queue.remove(&bot_nr);
                }
            }
        } else {
            self.bots.insert(bot_nr, vec![value]);
        }
    }

    fn bot_values(&self, bot_nr: u32) -> Vec<u32> {
        if let Some(value) = self.bots.get(&bot_nr) {
            value.clone()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
enum Target {
    Bot(u32),
    Output(u32),
}

#[derive(Debug, Clone)]
enum Instruction {
    ValueToBot((u32, u32)),
    BotTo((u32, Target, Target)),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn part1_examples() {
        // bot number `2` is responsible for comparing value-`5` microchips with value-`2` microchips
        assert_eq!(2, Factory::look_for(&parse_input(EXAMPLE), 2, 5).unwrap());
    }

    #[test]
    fn part2_examples() {
        // In the end, output bin `0` contains a value-`5` microchip, output bin `1` contains a
        // value-`2` microchip, and output bin `2` contains a value-`3` microchip
        assert_eq!(2 * 3 * 5, part2(&parse_input(EXAMPLE)));
    }
}
