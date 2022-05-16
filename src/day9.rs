//! # [Day 9: Explosives in Cyberspace](https://adventofcode.com/2016/day/9)
//!
//! Wandering around a secure area, you come across a datalink port to a new part of the network.
//! After briefly scanning it for interesting files, you find one file in particular that catches
//! your attention. It's compressed with an experimental format, but fortunately, the documentation
//! for the format is nearby.
//!
//! The format compresses a sequence of characters. Whitespace is ignored. To indicate that some
//! sequence should be repeated, a marker is added to the file, like `(10x2)`. To decompress this
//! marker, take the subsequent `10` characters and repeat them `2` times. Then, continue reading
//! the file after the repeated data. The marker itself is not included in the decompressed output.
//!
//! If parentheses or other characters appear within the data referenced by a marker, that's
//! okay - treat it like normal data, not a marker, and then resume looking for markers after
//! the decompressed section.
//!
//! For example:
//!
//! -   `ADVENT` contains no markers and decompresses to itself with no changes, resulting in a
//!     decompressed length of `6`.
//! -   `A(1x5)BC` repeats only the `B` a total of `5` times, becoming `ABBBBBC` for a
//!     decompressed length of `7`.
//! -   `(3x3)XYZ` becomes `XYZXYZXYZ` for a
//!     decompressed length of `9`.
//! -   `A(2x2)BCD(2x2)EFG` doubles the `BC` and `EF`, becoming `ABCBCDEFEFG` for a
//!     decompressed length of `11`.
//! -   `(6x1)(1x3)A` simply becomes `(1x3)A` - the `(1x3)` looks like a marker, but because it's
//!     within a data section of another marker, it is not treated any differently from the `A`
//!     that comes after it. It has a decompressed length of `6`.
//! -   `X(8x2)(3x3)ABCY` becomes `X(3x3)ABC(3x3)ABCY` (for a decompressed length of `18`),
//!     because the decompressed data from the `(8x2)` marker (the `(3x3)ABC`) is skipped and
//!     not processed further.
//!
//! **What is the decompressed length of the file (your puzzle input)? Don't count whitespace.**
//!
//! # Part Two
//!
//! Apparently, the file actually uses version two of the format.
//!
//! In version two, the only difference is that markers within decompressed data are decompressed.
//! This, the documentation explains, provides much more substantial compression capabilities,
//! allowing many-gigabyte files to be stored in only a few kilobytes.
//!
//! For example:
//!
//! -   `(3x3)XYZ` still becomes `XYZXYZXYZ`, as the decompressed section contains no markers.
//! -   `X(8x2)(3x3)ABCY` becomes `XABCABCABCABCABCABCY`, because the decompressed data from the
//!     `(8x2)` marker is then further decompressed, thus triggering the `(3x3)` marker twice
//!     for a total of six `ABC` sequences.
//! -   `(27x12)(20x12)(13x14)(7x10)(1x12)A` decompresses into a string of `A` repeated
//!     `241920` times.
//! -   `(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN` becomes `445` characters long.
//!
//! Unfortunately, the computer you brought probably doesn't have enough memory to actually
//! decompress the file; you'll have to come up with another way to get its decompressed length.
//!
//! **What is the decompressed length of the file using this improved format?**

/// Part 1: What is the decompressed length of the file (your puzzle input)? Don't count whitespace.
#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    decompress1(input).len()
}

/// Part 2: What is the decompressed length of the file using this improved format?
#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    decompress2(input).len()
}

fn parse_marker(input: &str) -> (usize, usize) {
    let parts: Vec<usize> = input.split('x').map(|n| n.parse().unwrap()).collect();
    (parts[0], parts[1])
}

#[allow(clippy::unnecessary_unwrap)]
fn decompress1(input: &str) -> String {
    let mut out = String::new();
    let mut marker_start: Option<usize> = None;
    let mut marker_stop: Option<usize> = None;
    for (idx, char) in input.chars().enumerate() {
        if marker_start.is_some() && marker_stop.is_none() && char == ')' {
            let (len, cnt) = parse_marker(&input[marker_start.unwrap() + 1..idx]);
            for _ in 0..cnt {
                out += &input[idx + 1..(idx + 1 + len)];
            }
            marker_stop = Some(idx + len);
        } else if marker_start.is_none() && char == '(' {
            marker_start = Some(idx);
        } else if marker_stop.is_some() && idx == marker_stop.unwrap() {
            marker_start = None;
            marker_stop = None;
        } else if marker_start.is_none() {
            out += &char.to_string();
        }
    }
    out
}

fn decompress2(input: &str) -> String {
    let mut str = decompress1(input);
    while str.contains('(') {
        let newstr = decompress1(&str);
        if str.len() == newstr.len() {
            break;
        }
        str = newstr;
    }
    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        //  `ADVENT` contains no markers and decompresses to itself with no changes, resulting in a
        //  decompressed length of `6`.
        assert_eq!("ADVENT", decompress1("ADVENT"));
        assert_eq!(6, part1("ADVENT"));

        //  `A(1x5)BC` repeats only the `B` a total of `5` times, becoming `ABBBBBC` for a
        //  decompressed length of `7`.
        assert_eq!("ABBBBBC", decompress1("A(1x5)BC"));
        assert_eq!(7, part1("A(1x5)BC"));

        //  `(3x3)XYZ` becomes `XYZXYZXYZ` for a
        //  decompressed length of `9`.
        assert_eq!("XYZXYZXYZ", decompress1("(3x3)XYZ"));
        assert_eq!(9, part1("(3x3)XYZ"));

        //  `A(2x2)BCD(2x2)EFG` doubles the `BC` and `EF`, becoming `ABCBCDEFEFG` for a
        //  decompressed length of `11`.
        assert_eq!("ABCBCDEFEFG", decompress1("A(2x2)BCD(2x2)EFG"));
        assert_eq!(11, part1("A(2x2)BCD(2x2)EFG"));

        // `(6x1)(1x3)A` simply becomes `(1x3)A` - the `(1x3)` looks like a marker, but because
        // it's  within a data section of another marker, it is not treated any differently from
        // the `A` that comes after it. It has a decompressed length of `6`.
        assert_eq!("(1x3)A", decompress1("(6x1)(1x3)A"));
        assert_eq!(6, part1("(6x1)(1x3)A"));

        //  `X(8x2)(3x3)ABCY` becomes `X(3x3)ABC(3x3)ABCY` (for a decompressed length of `18`),
        //  because the decompressed data from the `(8x2)` marker (the `(3x3)ABC`) is skipped and
        //  not processed further.
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress1("X(8x2)(3x3)ABCY"));
        assert_eq!(18, part1("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn part2_examples() {
        // `(3x3)XYZ` still becomes `XYZXYZXYZ`, as the decompressed section contains no markers.
        assert_eq!("XYZXYZXYZ", decompress2("(3x3)XYZ"));
        // `X(8x2)(3x3)ABCY` becomes `XABCABCABCABCABCABCY`, because the decompressed data from the
        // `(8x2)` marker is then further decompressed, thus triggering the `(3x3)` marker twice
        // for a total of six `ABC` sequences.
        assert_eq!("XABCABCABCABCABCABCY", decompress2("X(8x2)(3x3)ABCY"));
        // `(27x12)(20x12)(13x14)(7x10)(1x12)A` decompresses into a string of `A` repeated
        // `241920` times.
        assert_eq!(
            241920,
            decompress2("(27x12)(20x12)(13x14)(7x10)(1x12)A").len()
        );
        // `(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN` becomes `445` characters long.
        assert_eq!(
            445,
            decompress2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN").len()
        );
    }
}
