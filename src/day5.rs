//! # [Day 5: How About a Nice Game of Chess?](https://adventofcode.com/2016/day/5)
//!
//! You are faced with a security door designed by Easter Bunny engineers that seem to have
//! acquired most of their security knowledge by watching
//! [hacking](https://en.wikipedia.org/wiki/Hackers_(film))
//! [movies](https://en.wikipedia.org/wiki/WarGames).
//!
//! The eight-character password for the door is generated one character at a time by finding
//! the [MD5](https://en.wikipedia.org/wiki/MD5) hash of some Door ID (your puzzle input) and
//! an increasing integer index (starting with `0`).
//!
//! A hash indicates the next character in the password if its
//! [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal) representation starts with five zeroes.
//! If it does, the sixth character in the hash is the next character of the password.
//!
//! For example, if the Door ID is `abc`:
//!
//! -   The first index which produces a hash that starts with five zeroes is `3231929`, which we
//!     find by hashing `abc3231929`; the sixth character of the hash, and thus the first character
//!     of the password, is `1`.
//! -   `5017308` produces the next interesting hash, which starts with `000008f82...`, so the
//!     second character of the password is `8`.
//! -   The third time a hash starts with five zeroes is for `abc5278568`, discovering the
//!     character `f`.
//!
//! In this example, after continuing this search a total of eight times,
//! the password is `18f47a30`.
//!
//! **Given the actual Door ID, what is the password?**
//!
//! # Part Two
//!
//! As the door slides open, you are presented with a second door that uses a slightly more
//! inspired security mechanism. Clearly unimpressed by the last version (in what movie is the
//! password decrypted in order?!), the Easter Bunny engineers have worked out
//! [a better solution](https://www.youtube.com/watch?v=NHWjlCaIrQo&t=25).
//!
//! Instead of simply filling in the password from left to right, the hash now also indicates the
//! position within the password to fill. You still look for hashes that begin with five zeroes;
//! however, now, the sixth character represents the position (`0`-`7`), and the seventh character
//! is the character to put in that position.
//!
//! A hash result of `000001f` means that `f` is the second character in the password. Use only the
//! first result for each position, and ignore invalid positions.
//!
//! For example, if the Door ID is `abc`:
//!
//! -   The first interesting hash is from `abc3231929`, which produces `0000015...`; so, `5` goes
//!     in position `1`: `_5______`.
//! -   In the previous method, `5017308` produced an interesting hash; however, it is ignored,
//!     because it specifies an invalid position (`8`).
//! -   The second interesting hash is at index `5357525`, which produces `000004e...`; so, `e`
//!     goes in position `4`: `_5__e___`.
//!
//! You almost choke on your popcorn as the final character falls into place, producing the
//! password `05ace8e3`.
//!
//! **Given the actual Door ID and this new method, what is the password?**
//! Be extra proud of your solution if it uses a cinematic "decrypting" animation.

use crypto::digest::Digest;

/// Part 1: Given the actual Door ID, what is the password?
#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let mut buffer = String::new();
    let mut start = 0;
    for _ in 0..8 {
        let (next, md5) = md5_suffix_increment_until(input, start, |output| {
            let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
            first_five == 0
        });
        let md5 = format!("{:02x?}", md5[2]);
        let sixth = &md5[1..2];
        buffer += sixth;
        start = next + 1;
    }
    buffer
}

/// Part 2: Given the actual Door ID and this new method, what is the password?
#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let mut password: Vec<char> = vec![' '; 8];
    let mut start = 0;
    while password.contains(&' ') {
        let (next, md5) = md5_suffix_increment_until(input, start, |output| {
            let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
            first_five == 0
        });
        let md5a = format!("{:02x?}", md5[2]);
        let md5b = format!("{:02x?}", md5[3]);
        let char_to_put = &md5b[0..1].chars().next().unwrap();
        let position = &md5a[1..2];
        match position {
            "0" => {
                if password[0] == ' ' {
                    password[0] = *char_to_put
                }
            }
            "1" => {
                if password[1] == ' ' {
                    password[1] = *char_to_put
                }
            }
            "2" => {
                if password[2] == ' ' {
                    password[2] = *char_to_put
                }
            }
            "3" => {
                if password[3] == ' ' {
                    password[3] = *char_to_put
                }
            }
            "4" => {
                if password[4] == ' ' {
                    password[4] = *char_to_put
                }
            }
            "5" => {
                if password[5] == ' ' {
                    password[5] = *char_to_put
                }
            }
            "6" => {
                if password[6] == ' ' {
                    password[6] = *char_to_put
                }
            }
            "7" => {
                if password[7] == ' ' {
                    password[7] = *char_to_put
                }
            }
            _ => {}
        }
        start = next + 1;
    }
    String::from_iter(password)
}

/// increments a counter starting at 0 which is appended to `input` until `test` returns
/// true for the md5 hash buffer, then returns the counter
fn md5_suffix_increment_until(
    input: &str,
    start: u64,
    test: fn(&[u8; 16]) -> bool,
) -> (u64, [u8; 16]) {
    let mut hasher = crypto::md5::Md5::new();
    let mut output = [0; 16]; // An MD5 is 16 bytes
    for i in start..u64::MAX {
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut output);
        if test(&output) {
            return (i, output);
        }
        hasher.reset();
    }
    (0, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "abc";

    #[test]
    fn part1_examples() {
        // In this example, after continuing this search a total of eight times, the password is `18f47a30`.
        assert_eq!("18f47a30", part1(EXAMPLE));
    }

    #[test]
    fn part2_examples() {
        // You almost choke on your popcorn as the final character falls into place, producing the
        // password `05ace8e3`.
        assert_eq!("05ace8e3", part2(EXAMPLE));
    }
}
