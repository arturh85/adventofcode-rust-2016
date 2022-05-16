//! # [Day 4: Security Through Obscurity](https://adventofcode.com/2016/day/4)
//!
//! Finally, you come across an information kiosk with a list of rooms. Of course, the list
//! is encrypted and full of decoy data, but the instructions to decode the list are barely hidden
//! nearby. Better remove the decoy data first.
//!
//! Each room consists of an encrypted name (lowercase letters separated by dashes) followed by
//! a dash, a sector ID, and a checksum in square brackets.
//!
//! A room is real (not a decoy) if the checksum is the five most common letters in the
//! encrypted name, in order, with ties broken by alphabetization.
//!
//! For example:
//!
//! -   `aaaaa-bbb-z-y-x-123[abxyz]` is a real room because the most common letters are `a` (5),
//!     `b` (3), and then a tie between `x`, `y`, and `z`, which are listed alphabetically.
//! -   `a-b-c-d-e-f-g-h-987[abcde]` is a real room because although the letters are all tied
//!     (1 of each), the first five are listed alphabetically.
//! -   `not-a-real-room-404[oarel]` is a real room.
//! -   `totally-real-room-200[decoy]` is not.
//!
//! Of the real rooms from the list above, the sum of their sector IDs is `1514`.
//!
//! **What is the sum of the sector IDs of the real rooms?**
//!
//! # Part Two
//!
//! With all the decoy data out of the way, it's time to decrypt this list and get moving.
//!
//! The room names are encrypted by a state-of-the-art
//! [shift cipher](https://en.wikipedia.org/wiki/Caesar_cipher), which is nearly unbreakable
//! without the right software. However, the information kiosk designers at Easter Bunny HQ were
//! not expecting to deal with a master cryptographer like yourself.
//!
//! To decrypt a room name, rotate each letter forward through the alphabet a number of times
//! equal to the room's sector ID. `A` becomes `B`, `B` becomes `C`, `Z` becomes `A`, and so on.
//! Dashes become spaces.
//!
//! For example, the real name for `qzmt-zixmtkozy-ivhz-343` is `very encrypted name`.
//!
//! **What is the sector ID of the room where North Pole objects are stored?**

use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Room> {
    let re = Regex::new(r"^(?P<name>[-\w]+)-(?P<sector_id>\d+)\[(?P<checksum>\w+)\]$").unwrap();
    input
        .lines()
        .map(|line| {
            let matches = re.captures(line).expect("failed to parse");
            let name = matches.name("name").unwrap().as_str().to_string();
            let checksum = matches.name("checksum").unwrap().as_str().to_string();
            let sector_id = matches.name("sector_id").unwrap().as_str().parse().unwrap();
            Room {
                name,
                sector_id,
                checksum,
            }
        })
        .collect()
}

/// Part 1: What is the sum of the sector IDs of the real rooms?
#[aoc(day4, part1)]
fn part1(input: &[Room]) -> u32 {
    input
        .iter()
        .filter(|room| room.is_valid())
        .map(|room| room.sector_id)
        .sum()
}

/// Part 2: What is the sector ID of the room where North Pole objects are stored?
#[aoc(day4, part2)]
fn part2(input: &[Room]) -> u32 {
    input
        .iter()
        .find(|room| decrypt(&room.name, room.sector_id).eq("northpole object storage"))
        .expect("found nothing")
        .sector_id
}

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    /// A room is real (not a decoy) if the checksum is the five most common letters in the
    /// encrypted name, in order, with ties broken by alphabetization.
    fn is_valid(&self) -> bool {
        let mut count_map: HashMap<char, u32> = HashMap::new();
        for c in self.name.replace('-', "").chars() {
            if let Entry::Vacant(e) = count_map.entry(c) {
                e.insert(1);
            } else {
                let val = count_map.get_mut(&c).unwrap();
                *val += 1;
            }
        }
        let counts: HashSet<u32> = count_map.values().copied().collect();
        let mut counts: Vec<u32> = counts.iter().copied().collect();
        counts.sort_unstable();
        counts.reverse();

        let mut checksum = String::new();

        for count in counts {
            let mut part = String::new();
            for c in count_map.keys() {
                if *count_map.get(c).unwrap() == count {
                    part += &c.to_string();
                }
            }
            checksum += &sort(&part);
            if checksum.len() > 5 {
                checksum = checksum[0..5].to_string();
            }
            if checksum.len() == 5 {
                break;
            }
        }
        self.checksum.eq(&checksum)
    }
}

fn decrypt(name: &str, sector_id: u32) -> String {
    let mut buffer = String::new();

    let a: u32 = 'a' as u32;
    let z: u32 = 'z' as u32;

    for c in name.chars() {
        if c == '-' {
            buffer += " ";
        } else {
            let mut n = c as u32 - a;
            n += sector_id;
            n %= z - a + 1;
            n += a;
            buffer += &(n as u8 as char).to_string();
        }
    }
    buffer
}

fn sort(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    String::from_iter(chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn part1_examples() {
        let examples = parse_input(EXAMPLE);

        // `aaaaa-bbb-z-y-x-123[abxyz]` is a real room because the most common letters are `a` (5),
        // `b` (3), and then a tie between `x`, `y`, and `z`, which are listed alphabetically.
        assert_eq!(true, examples[0].is_valid());
        // `a-b-c-d-e-f-g-h-987[abcde]` is a real room because although the letters are all tied
        // (1 of each), the first five are listed alphabetically.
        assert_eq!(true, examples[1].is_valid());
        // `not-a-real-room-404[oarel]` is a real room.
        assert_eq!(true, examples[2].is_valid());
        // `totally-real-room-200[decoy]` is not.
        assert_eq!(false, examples[3].is_valid());
    }

    #[test]
    fn part2_examples() {
        // For example, the real name for `qzmt-zixmtkozy-ivhz-343` is `very encrypted name`.
        assert_eq!("qzmt zixmtkozy ivhz", decrypt("qzmt-zixmtkozy-ivhz", 0));
        assert_eq!("very encrypted name", decrypt("qzmt-zixmtkozy-ivhz", 343));
    }
}
