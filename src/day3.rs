//! # [Day 3: Squares With Three Sides](https://adventofcode.com/2016/day/3)
//!
//! Now that you can think clearly, you move deeper into the labyrinth of hallways and office
//! furniture that makes up this part of Easter Bunny HQ. This must be a graphic design department;
//! the walls are covered in specifications for triangles.
//!
//! Or are they?
//!
//! The design document gives the side lengths of each triangle it describes, but... `5 10 25`?
//! Some of these aren't triangles. You can't help but mark the impossible ones.
//!
//! In a valid triangle, the sum of any two sides must be larger than the remaining side.
//! For example, the "triangle" given above is impossible, because `5 + 10` is not larger than `25`.
//!
//! **In your puzzle input, how many of the listed triangles are possible?**
//!
//! # Part Two
//!
//! Now that you've helpfully marked up their design documents, it occurs to you that triangles are
//! specified in groups of three vertically. Each set of three numbers in a column specifies a
//! triangle. Rows are unrelated.
//!
//! For example, given the following specification, numbers with the same hundreds digit would be
//! part of the same triangle:
//!
//! ```plain
//! 101 301 501
//! 102 302 502
//! 103 303 503
//! 201 401 601
//! 202 402 602
//! 203 403 603
//! ```
//!
//! **In your puzzle input, and instead reading by columns, how many of the listed triangles
//! are possible?**

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<[u64; 3]> {
    input
        .lines()
        .map(|line| {
            let mut triangle = [0u64; 3];
            let parts: Vec<u64> = line
                .trim()
                .replace("   ", " ")
                .replace("  ", " ")
                .split(' ')
                .map(|c| c.parse().unwrap())
                .collect();
            triangle[0] = parts[0];
            triangle[1] = parts[1];
            triangle[2] = parts[2];
            triangle
        })
        .collect()
}

/// Part 1: In your puzzle input, how many of the listed triangles are possible?
#[aoc(day3, part1)]
fn part1(input: &[[u64; 3]]) -> usize {
    input
        .iter()
        .filter(|triangle| valid_triange(triangle))
        .count()
}

/// Part 2: In your puzzle input, and instead reading by columns, how many of the listed triangles
/// are possible?
#[aoc(day3, part2)]
fn part2(input: &[[u64; 3]]) -> usize {
    rebuild_triangles(input)
        .iter()
        .filter(|triangle| valid_triange(triangle))
        .count()
}

// interpret triangles row-wise
#[allow(clippy::identity_op)]
fn rebuild_triangles(input: &[[u64; 3]]) -> Vec<[u64; 3]> {
    let mut triangles = Vec::new();

    for col in 0..3 {
        for row_block in 0..input.len() / 3 {
            let row = row_block * 3;
            let mut triangle = [0u64; 3];
            triangle[0] = input[row + 0][col];
            triangle[1] = input[row + 1][col];
            triangle[2] = input[row + 2][col];
            triangles.push(triangle);
        }
    }

    triangles
}

/// In a valid triangle, the sum of any two sides must be larger than the remaining side.
fn valid_triange(triangle: &[u64; 3]) -> bool {
    triangle[1] + triangle[2] > triangle[0]
        && triangle[0] + triangle[2] > triangle[1]
        && triangle[0] + triangle[1] > triangle[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input("5 10 25")), 0);
    }
}
