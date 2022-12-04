use aoc_runner_derive::aoc;
use eyre::{eyre, Result};

const CHAR_OFFSET: u32 = 9;
const UPPERCASE_OFFSET: u32 = 26;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    get_rucksack_items(input)
        .and_then(|items| get_total_priority_from_items(&items))
        .unwrap_or_default()
}

fn get_rucksack_items(text: &str) -> Result<Vec<char>> {
    let mut rucksack_items: Vec<char> = Vec::new();
    for line in text.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let common_item: Result<char> = get_common_item_in_both_halves(left, right);
        rucksack_items.push(common_item?);
    }

    Ok(rucksack_items)
}

fn get_common_item_in_both_halves(left: &str, right: &str) -> Result<char> {
    for char_left in left.chars() {
        for char_right in right.chars() {
            if char_left.eq(&char_right) {
                return Ok(char_left);
            }
        }
    }

    Err(eyre!(
        "Not common character found in both halves: '{}' and '{}'",
        left,
        right
    ))
}

fn get_total_priority_from_items(items: &[char]) -> Result<u32> {
    let list_of_priorities = items.iter().map(|item| get_priority_from_char(*item));
    let mut total: u32 = 0;
    for priority in list_of_priorities {
        total += priority?;
    }
    Ok(total)
}

fn get_priority_from_char(input_char: char) -> Result<u32> {
    match input_char.to_digit(36) {
        Some(val) => {
            if input_char.is_lowercase() {
                Ok(val - CHAR_OFFSET)
            } else {
                Ok(val + UPPERCASE_OFFSET - CHAR_OFFSET)
            }
        }
        None => Err(eyre!("Couldn't convert {} char to a digit", input_char)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcd", "xxax", 'a')]
    fn test_get_common_item_in_both_halves(
        #[case] left: &str,
        #[case] right: &str,
        #[case] expected: char,
    ) {
        assert_eq!(
            get_common_item_in_both_halves(left, right).unwrap(),
            expected
        );
    }

    #[test]
    fn test_get_rucksack_items() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
        let expected_result: Vec<char> = vec!['p', 'L', 'P', 'v', 't', 's'];

        let output_list = get_rucksack_items(input_str).unwrap();

        assert_eq!(output_list.len(), expected_result.len());

        let matching = output_list
            .iter()
            .zip(&expected_result)
            .filter(|&(a, b)| a.eq(b))
            .count();

        assert_eq!(matching, expected_result.len());
    }

    #[rstest]
    #[case('a', 1)]
    #[case('z', 26)]
    #[case('A', 27)]
    #[case('Z', 52)]
    fn test_chars(#[case] input_char: char, #[case] char_value: u32) {
        assert_eq!(get_priority_from_char(input_char).unwrap(), char_value);
    }

    #[test]
    fn test_get_total_priority_from_items() {
        let input_priorities = vec!['p', 'L', 'P', 'v', 't', 's'];

        let output_total = get_total_priority_from_items(&input_priorities);

        assert_eq!(output_total.unwrap(), 157)
    }
}
