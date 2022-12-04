use aoc_runner_derive::aoc;
use eyre::{eyre, Result};
use std::borrow::ToOwned;
use std::str::FromStr;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let result: Result<u32> = get_blocks_of_calories(input)
        .and_then(|blocks| get_max_block_of_calories(blocks.as_slice()));
    result.unwrap_or_default()
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
