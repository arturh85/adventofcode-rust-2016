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
    Factory::find_bot_comparing(input, 17, 61).unwrap()
}

/// Part 2: What do you get if you multiply together the values of one chip in each of
/// outputs 0, 1, and 2?
#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> u32 {
    let mut state = Factory::new();
    state.execute(input);
    (0..3_u32).map(|n| state.outputs.get(&n).unwrap()).product()
}

type BotId = u32;
type OutputId = u32;
type ChipId = u32;

struct Factory {
    bots: HashMap<BotId, Vec<ChipId>>,
    outputs: HashMap<OutputId, ChipId>,
    queue: HashMap<BotId, (Target, Target)>,
    look_for_comparision: Option<(ChipId, ChipId)>,
    bot_comparing: Option<BotId>,
}

impl Factory {
    fn new() -> Self {
        Factory {
            bots: HashMap::new(),
            outputs: HashMap::new(),
            queue: HashMap::new(),
            look_for_comparision: None,
            bot_comparing: None,
        }
    }

    fn find_bot_comparing(input: &[Instruction], a: ChipId, b: ChipId) -> Option<BotId> {
        let mut state = Factory::new();
        state.look_for_comparision = Some(if a < b { (a, b) } else { (b, a) });
        state.execute(input);
        state.bot_comparing
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::ValueToBot((bot_nr, value)) => self.value_to_bot(*bot_nr, *value),
                Instruction::BotTo((bot_nr, low_target, high_target)) => {
                    let bot_values = self.bot_chips(*bot_nr);
                    if bot_values.len() == 2 {
                        self.bot_to(*bot_nr, *low_target, *high_target);
                    } else {
                        self.queue.insert(*bot_nr, (*low_target, *high_target));
                    }
                }
            };
        }
    }

    fn bot_to(&mut self, bot_nr: BotId, low_target: Target, high_target: Target) {
        let bot_values = self.bot_chips(bot_nr);
        if bot_values.len() == 2 {
            let low = bot_values[0];
            let high = bot_values[1];
            if let Some((look_low, look_high)) = self.look_for_comparision {
                if low == look_low && high == look_high {
                    self.bot_comparing = Some(bot_nr);
                }
            }
            self.value_to_target(low_target, low);
            self.value_to_target(high_target, high);
            self.bots.remove(&bot_nr);
        }
    }

    fn value_to_target(&mut self, target: Target, value: ChipId) {
        match target {
            Target::Bot(bot_nr) => self.value_to_bot(bot_nr, value),
            Target::Output(output_nr) => {
                self.outputs.insert(output_nr, value);
            }
        }
    }

    fn value_to_bot(&mut self, bot_nr: BotId, value: ChipId) {
        let values = self.bots.entry(bot_nr).or_insert_with(Vec::new);
        values.push(value);
        values.sort_unstable();
        if values.len() == 2 {
            if let Some((low_target, high_target)) = self.queue.remove(&bot_nr) {
                self.bot_to(bot_nr, low_target, high_target);
            }
        }
    }

    fn bot_chips(&self, bot_nr: BotId) -> Vec<ChipId> {
        if let Some(value) = self.bots.get(&bot_nr) {
            value.clone()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Target {
    Bot(BotId),
    Output(OutputId),
}

#[derive(Debug, Clone)]
enum Instruction {
    ValueToBot((BotId, ChipId)),
    BotTo((BotId, Target, Target)),
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
        assert_eq!(
            2,
            Factory::find_bot_comparing(&parse_input(EXAMPLE), 2, 5).unwrap()
        );
    }

    #[test]
    fn part2_examples() {
        // In the end, output bin `0` contains a value-`5` microchip, output bin `1` contains a
        // value-`2` microchip, and output bin `2` contains a value-`3` microchip
        assert_eq!(2 * 3 * 5, part2(&parse_input(EXAMPLE)));
    }
}
