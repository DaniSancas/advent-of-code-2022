use aoc_runner_derive::aoc;
use eyre::Result;
use regex::{Captures, Regex};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    compile_regex(input).unwrap_or_default()
}

fn compile_regex(input: &str) -> Result<u32> {
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
    let total_ranges = regex
        .captures_iter(input)
        .into_iter()
        .map(|group| get_ranges_fully_contained(group))
        .map(|group| group.unwrap_or_default())
        .sum();

    Ok(total_ranges)
}

fn get_ranges_fully_contained(group: Captures) -> Result<u32> {
    let left_start = group[1].parse::<u32>()?;
    let left_end = group[2].parse::<u32>()?;
    let right_start = group[3].parse::<u32>()?;
    let right_end = group[4].parse::<u32>()?;

    if (left_start <= right_start && left_end >= right_end)
        || (right_start <= left_start && right_end >= left_end)
    {
        Ok(1)
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n50-60,51-59\n";

        let expected = 3;

        let output = part1(input);

        assert_eq!(output, expected);
    }
}
