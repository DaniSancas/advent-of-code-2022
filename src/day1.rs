use aoc_runner_derive::aoc;
use eyre::{eyre, Result};
use std::borrow::ToOwned;
use std::str::FromStr;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> String {
    let result = get_blocks_of_calories(input)
        .and_then(|blocks| get_max_block_of_calories(blocks.as_slice()));

    match result {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> String {
    let result = get_blocks_of_calories(input)
        .and_then(|mut blocks| get_top3_block_of_calories_sum(blocks.as_mut_slice()));

    match result {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    }
}

fn get_blocks_of_calories(text: &str) -> Result<Vec<u32>> {
    let mut blocks_of_calories: Vec<u32> = Vec::new();
    let mut calories: u32 = 0;

    for line in text.lines() {
        if line.is_empty() {
            // Empty line means a block final
            blocks_of_calories.push(calories);
            calories = 0;
        } else {
            // If not empty, accumulate calories of the block
            let line_of_calories: u32 = FromStr::from_str(line)?;
            calories += line_of_calories;
        }
    }

    Ok(blocks_of_calories)
}

fn get_max_block_of_calories(blocks: &[u32]) -> Result<u32> {
    blocks
        .iter()
        .max()
        .map(ToOwned::to_owned)
        .ok_or_else(|| eyre!("Couldn't get the max calorie block."))
}

fn get_top3_block_of_calories_sum(blocks: &mut [u32]) -> Result<u32> {
    blocks.sort_unstable();
    Ok(blocks.iter().rev().take(3).sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_top3_block_of_calories_sum() {
        let mut input_blocks: Vec<u32> = vec![14, 56, 2, 100, 70];
        let expected_sum: u32 = 100 + 70 + 56;

        let output_sum = get_top3_block_of_calories_sum(input_blocks.as_mut_slice());

        assert_eq!(output_sum.unwrap(), expected_sum);
    }
}
