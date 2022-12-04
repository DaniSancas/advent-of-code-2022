use aoc_runner_derive::aoc;
use eyre::Result;
use regex::{Captures, Regex};

struct Ranges {
    left_start: u32,
    left_end: u32,
    right_start: u32,
    right_end: u32,
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    compile_regex(input, get_ranges_fully_contained).unwrap_or_default()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    compile_regex(input, get_ranges_partially_contained).unwrap_or_default()
}

fn compile_regex(input: &str, ranges_contained_function: fn(Result<Ranges>) -> Result<u32>) -> Result<u32> {
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
    let total_ranges = regex
        .captures_iter(input)
        .into_iter()
        .map(|group| group_to_ranges(&group))
        .map(ranges_contained_function)
        .map(|group| group.unwrap_or_default())
        .sum();

    Ok(total_ranges)
}

fn group_to_ranges(group: &Captures) -> Result<Ranges> {
    Ok(Ranges {
        left_start: group[1].parse::<u32>()?,
        left_end: group[2].parse::<u32>()?,
        right_start: group[3].parse::<u32>()?,
        right_end: group[4].parse::<u32>()?,
    })
}

fn get_ranges_fully_contained(ranges_maybe: Result<Ranges>) -> Result<u32> {
    let ranges = ranges_maybe?;

    if (ranges.left_start <= ranges.right_start && ranges.left_end >= ranges.right_end)
        || (ranges.right_start <= ranges.left_start && ranges.right_end >= ranges.left_end)
    {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn get_ranges_partially_contained(ranges_maybe: Result<Ranges>) -> Result<u32> {
    let ranges = ranges_maybe?;

    if (ranges.left_start <= ranges.right_start && ranges.left_end >= ranges.right_start)
        || (ranges.right_start <= ranges.left_start && ranges.right_end >= ranges.left_start)
    {
        Ok(1)
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n50-60,51-59\n";

        let expected = 3;

        let output = part1(input);

        assert_eq!(output, expected);
    }

    #[rstest]
    #[case(2, 4, 6, 8, 0)]
    #[case(2, 8, 3, 7, 1)]
    #[case(5, 10, 6, 9, 1)]
    fn test_get_ranges_fully_contained(
        #[case] ls: u32,
        #[case] le: u32,
        #[case] rs: u32,
        #[case] re: u32,
        #[case] expected: u32,
    ) {
        let input: Result<Ranges> = Ok(Ranges {
            left_start: ls,
            left_end: le,
            right_start: rs,
            right_end: re,
        });

        let output = get_ranges_fully_contained(input);

        assert_eq!(output.unwrap(), expected);
    }

    #[rstest]
    #[case(2, 4, 6, 8, 0)]
    #[case(2, 4, 5, 7, 0)]
    #[case(5, 7, 7, 9, 1)]
    #[case(2, 8, 3, 7, 1)]
    #[case(6, 7, 4, 6, 1)]
    #[case(2, 6, 4, 8, 1)]
    #[case(5, 10, 6, 9, 1)]
    fn test_get_ranges_partially_contained(
        #[case] ls: u32,
        #[case] le: u32,
        #[case] rs: u32,
        #[case] re: u32,
        #[case] expected: u32,
    ) {
        let input: Result<Ranges> = Ok(Ranges {
            left_start: ls,
            left_end: le,
            right_start: rs,
            right_end: re,
        });

        let output = get_ranges_partially_contained(input);

        assert_eq!(output.unwrap(), expected);
    }
}
