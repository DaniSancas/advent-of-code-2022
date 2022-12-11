use std::collections::HashSet;

use aoc_runner_derive::aoc;
use eyre::{eyre, Result};

#[aoc(day6, part1)]
pub fn part1(input: &str) -> String {
    match get_first_marker(input, 4) {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    }
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> String {
    match get_first_marker(input, 14) {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    }
}

fn get_first_marker(line: &str, size: usize) -> Result<u32> {
    let result = Err(eyre!("Couldn't get first marker."));

    for slice in 0..line.len() {
        let end = slice + size;
        match line.get(slice..end) {
            Some(marker) if marker.len() == size => {
                let converted = marker.chars().collect::<HashSet<char>>();
                if converted.len() == size {
                    return Ok(end as u32);
                } else {
                    continue;
                }
            }
            _ => return result,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    #[case("zcfw", 4)]
    fn test_get_first_marker_part1(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(get_first_marker(line, 4).unwrap(), expected)
    }

    #[rstest]
    #[case("zcfz")]
    #[case("zcf")]
    fn test_get_first_marker_fail_part1(#[case] line: &str) {
        assert!(get_first_marker(line, 4).is_err())
    }
}
