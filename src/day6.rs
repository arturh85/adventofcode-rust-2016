//! # [Day 6: Signals and Noise](https://adventofcode.com/2016/day/6)
//!
//! Something is jamming your communications with Santa. Fortunately, your signal is only partially
//! jammed, and protocol in situations like this is to switch to a simple
//! [repetition code](https://en.wikipedia.org/wiki/Repetition_code) to get the message through.
//!
//! In this model, the same message is sent repeatedly. You've recorded the repeating message signal
//! (your puzzle input), but the data seems quite corrupted - almost too badly to recover. Almost.
//!
//! All you need to do is figure out which character is most frequent for each position.
//! For example, suppose you had recorded the following messages:
//!
//! ```plain
//! eedadn
//! drvtee
//! eandsr
//! raavrd
//! atevrs
//! tsrnev
//! sdttsa
//! rasrtv
//! nssdts
//! ntnada
//! svetve
//! tesnvt
//! vntsnd
//! vrdear
//! dvrsen
//! enarar
//! ```
//!
//! The most common character in the first column is `e`; in the second, `a`; in the third, `s`,
//! and so on. Combining these characters returns the error-corrected message, `easter`.
//!
//! **Given the recording in your puzzle input, what is the error-corrected version of the
//! message being sent?**
//!
//! # Part Two
//!
//! Of course, that would be the message - if you hadn't agreed to use a modified repetition
//! code instead.
//!
//! In this modified code, the sender instead transmits what looks like random data, but for each
//! character, the character they actually want to send is slightly less likely than the others.
//! Even after signal-jamming noise, you can look at the letter distributions in each column and
//! choose the least common letter to reconstruct the original message.
//!
//! In the above example, the least common character in the first column is `a`; in the
//! second, `d`, and so on. Repeating this process for the remaining characters produces the
//! original message, `advent`.
//!
//! Given the recording in your puzzle input and this new decoding methodology, what is the original
//! message that Santa is trying to send?

use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Part 1: Given the recording in your puzzle input, what is the error-corrected version of the
/// message being sent?
#[aoc(day6, part1)]
fn part1(input: &str) -> String {
    decode(input, true)
}

/// Part 2
#[aoc(day6, part2)]
fn part2(input: &str) -> String {
    decode(input, false)
}

fn decode(input: &str, most_common: bool) -> String {
    let cols = by_column(input);
    let mut out = String::new();

    for col in cols {
        let mut count_map: HashMap<char, u32> = HashMap::new();
        for c in col.chars() {
            if let Entry::Vacant(e) = count_map.entry(c) {
                e.insert(1);
            } else {
                let val = count_map.get_mut(&c).unwrap();
                *val += 1;
            }
        }
        let mut counts: Vec<u32> = count_map.values().copied().collect();
        counts.sort_unstable();
        if most_common {
            counts.reverse();
        }

        let char = count_map
            .keys()
            .find(|key| count_map[key] == counts[0])
            .unwrap();
        out += &char.to_string();
    }

    out
}

fn by_column(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut first = true;
    for line in input.lines() {
        if first {
            for _ in line.chars() {
                out.push(String::new());
            }
            first = false;
        }
        for (idx, char) in line.chars().enumerate() {
            out[idx] += &char.to_string();
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn part1_examples() {
        // The most common character in the first column is `e`; in the second, `a`; in the third,
        // `s`, and so on. Combining these characters returns the error-corrected message, `easter`.
        assert_eq!("easter", part1(EXAMPLE));
    }

    #[test]
    fn part2_examples() {
        // In the above example, the least common character in the first column is `a`; in the
        // second, `d`, and so on. Repeating this process for the remaining characters produces the
        // original message, `advent`.
        assert_eq!("advent", part2(EXAMPLE));
    }
}
