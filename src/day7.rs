//! # [Day 7: Internet Protocol Version 7](https://adventofcode.com/2016/day/7)
//!
//! While snooping around the local network of EBHQ, you compile a list of
//! [IP addresses](https://en.wikipedia.org/wiki/IP_address) (they're IPv7, of course;
//! [IPv6](https://en.wikipedia.org/wiki/IPv6) is much too limited). You'd like to figure out which
//! IPs support TLS (transport-layer snooping).
//!
//! An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An ABBA is any
//! four-character sequence which consists of a pair of two different characters followed by the
//! reverse of that pair, such as `xyyx` or `abba`. However, the IP also must not have an ABBA
//! within any hypernet sequences, which are contained by square brackets.
//!
//! For example:
//!
//! -   `abba[mnop]qrst` supports TLS (`abba` outside square brackets).
//! -   `abcd[bddb]xyyx` does not support TLS (`bddb` is within square brackets, even though `xyyx`
//!     is outside square brackets).
//! -   `aaaa[qwer]tyui` does not support TLS (`aaaa` is invalid; the interior characters must
//!     be different).
//! -   `ioxxoj[asdfgh]zxcvbn` supports TLS (`oxxo` is outside square brackets, even though it's
//!     within a larger string).
//!
//! **How many IPs in your puzzle input support TLS?**
//!
//! # Part Two
//!
//! You would also like to know which IPs support SSL (super-secret listening).
//!
//! An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere in the supernet
//! sequences (outside any square bracketed sections), and a corresponding Byte Allocation Block,
//! or BAB, anywhere in the hypernet sequences. An ABA is any three-character sequence which
//! consists of the same character twice with a different character between them, such as `xyx`
//! or `aba`. A corresponding BAB is the same characters but in reversed positions: `yxy` and `bab`,
//! respectively.
//!
//! For example:
//!
//! -   `aba[bab]xyz` supports SSL (`aba` outside square brackets with corresponding `bab`
//!     within square brackets).
//! -   `xyx[xyx]xyx` does not support SSL (`xyx`, but no corresponding `yxy`).
//! -   `aaa[kek]eke` supports SSL (`eke` in supernet with corresponding `kek` in hypernet;
//!     the `aaa` sequence is not related, because the interior character must be different).
//! -   `zazbz[bzb]cdb` supports SSL (`zaz` has no corresponding `aza`, but `zbz` has a
//!     corresponding `bzb`, even though `zaz` and `zbz` overlap).
//!
//! **How many IPs in your puzzle input support SSL?**

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Ipv7Address> {
    input
        .lines()
        .map(|line| {
            let mut normal = Vec::new();
            let mut hypernet = Vec::new();

            let parts: Vec<String> = line
                .replace("[", "|[")
                .replace("]", "]|")
                .split('|')
                .map(|p| p.to_string())
                .collect();
            for part in parts {
                if part.starts_with("[") {
                    hypernet.push(part.replace("[", "").replace("]", ""));
                } else {
                    normal.push(part);
                }
            }
            Ipv7Address {
                supernets: normal,
                hypernets: hypernet,
            }
        })
        .collect()
}

/// Part 1: How many IPs in your puzzle input support TLS?
#[aoc(day7, part1)]
fn part1(input: &Vec<Ipv7Address>) -> usize {
    input
        .iter()
        .filter(|address| address.is_tls_supported())
        .count()
}

/// Part 2: How many IPs in your puzzle input support SSL?
#[aoc(day7, part2)]
fn part2(input: &Vec<Ipv7Address>) -> usize {
    input
        .iter()
        .filter(|address| address.is_ssl_supported())
        .count()
}

struct Ipv7Address {
    supernets: Vec<String>,
    hypernets: Vec<String>,
}

impl Ipv7Address {
    /// An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An ABBA is any
    /// four-character sequence which consists of a pair of two different characters followed by the
    /// reverse of that pair, such as `xyyx` or `abba`. However, the IP also must not have an ABBA
    /// within any hypernet sequences, which are contained by square brackets.
    fn is_tls_supported(&self) -> bool {
        for hypernet in &self.hypernets {
            if is_abba(hypernet) {
                return false;
            }
        }
        for supernet in &self.supernets {
            if is_abba(supernet) {
                return true;
            }
        }
        false
    }

    /// An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere in the supernet
    /// sequences (outside any square bracketed sections), and a corresponding Byte Allocation Block,
    /// or BAB, anywhere in the hypernet sequences. An ABA is any three-character sequence which
    /// consists of the same character twice with a different character between them, such as `xyx`
    /// or `aba`. A corresponding BAB is the same characters but in reversed positions: `yxy` and `bab`,
    /// respectively.
    fn is_ssl_supported(&self) -> bool {
        let mut supernet_aba: Vec<String> = Vec::new();
        for supernet in &self.supernets {
            supernet_aba.append(&mut is_aba(supernet));
        }
        for hypernet in &self.hypernets {
            for aba in &supernet_aba {
                if hypernet.contains(aba) {
                    return true;
                }
            }
        }
        false
    }
}

/// An ABA is any three-character sequence which consists of the same character twice with a
/// different character between them, such as `xyx` or `aba`.
fn is_aba(input: &str) -> Vec<String> {
    let mut aba_list = Vec::new();
    let input: Vec<char> = input.chars().map(|c| c).collect();
    for window in input.windows(3) {
        let window = String::from_iter(window);
        let chars: Vec<char> = window.chars().map(|c| c).collect();
        if chars[0] != chars[1] && chars[0] == chars[2] {
            let mut bab: Vec<char> = Vec::new();
            bab.push(chars[1]);
            bab.push(chars[0]);
            bab.push(chars[1]);
            aba_list.push(String::from_iter(bab));
        }
    }
    aba_list
}

fn is_abba(input: &str) -> bool {
    let input: Vec<char> = input.chars().map(|c| c).collect();
    for window in input.windows(4) {
        let window = String::from_iter(window);
        let chars: Vec<char> = window.chars().map(|c| c).collect();
        if chars[0] != chars[1] && is_palindrome(&window) {
            return true;
        }
    }
    false
}

// https://github.com/bluejekyll/palindrome-rs/blob/master/src/lib.rs
fn is_palindrome(phrase: &str) -> bool {
    // get the chars iterator and associated index
    phrase
        .char_indices()
        .filter(|&(_, c)| c.is_alphabetic())
        // zip with the second half...
        .zip(
            phrase
                .char_indices()
                // which needs to be reversed...
                .rev()
                // and filter out bad cars
                .filter(|&(_, c)| c.is_alphabetic()),
        )
        // accept all input until the indexes have crossed
        .take_while(|&((first_count, _), (last_count, _))| first_count < last_count)
        // check that all the chars from the beginning and end match
        .all(|((_, first_char), (_, last_char))| {
            first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        // `abba[mnop]qrst` supports TLS (`abba` outside square brackets).
        assert_eq!(true, parse_input("abba[mnop]qrst")[0].is_tls_supported());
        // `abcd[bddb]xyyx` does not support TLS (`bddb` is within square brackets, even though
        // `xyyx` is outside square brackets).
        assert_eq!(false, parse_input("abcd[bddb]xyyx")[0].is_tls_supported());
        // `aaaa[qwer]tyui` does not support TLS (`aaaa` is invalid; the interior characters must
        // be different).
        assert_eq!(false, parse_input("aaaa[qwer]tyui")[0].is_tls_supported());
        // `ioxxoj[asdfgh]zxcvbn` supports TLS (`oxxo` is outside square brackets, even though it's
        // within a larger string).
        assert_eq!(
            true,
            parse_input("ioxxoj[asdfgh]zxcvbn")[0].is_tls_supported()
        );
    }

    #[test]
    fn part2_examples() {
        // `aba[bab]xyz` supports SSL (`aba` outside square brackets with corresponding `bab`
        // within square brackets).
        assert_eq!(true, parse_input("aba[bab]xyz")[0].is_ssl_supported());
        // `xyx[xyx]xyx` does not support SSL (`xyx`, but no corresponding `yxy`).
        assert_eq!(false, parse_input("xyx[xyx]xyx")[0].is_ssl_supported());
        // `aaa[kek]eke` supports SSL (`eke` in supernet with corresponding `kek` in hypernet;
        // the `aaa` sequence is not related, because the interior character must be different).
        assert_eq!(true, parse_input("aaa[kek]eke")[0].is_ssl_supported());
        // `zazbz[bzb]cdb` supports SSL (`zaz` has no corresponding `aza`, but `zbz` has a
        // corresponding `bzb`, even though `zaz` and `zbz` overlap).
        assert_eq!(true, parse_input("zazbz[bzb]cdb")[0].is_ssl_supported());
    }
}
