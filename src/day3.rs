use aoc_runner_derive::aoc;
use eyre::{eyre, Result};

const CHAR_OFFSET: u32 = 9;
const UPPERCASE_OFFSET: u32 = 26;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> String {
    let result =
        get_rucksack_items_part1(input).and_then(|items| get_total_priority_from_items(&items));

    match result {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> String {
    let result =
        get_rucksack_items_part2(input).and_then(|items| get_total_priority_from_items(&items));

    match result {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    }
}

fn get_rucksack_items_part1(text: &str) -> Result<Vec<char>> {
    let mut rucksack_items: Vec<char> = Vec::new();
    for line in text.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let common_item: Result<char> = get_common_item_in_both_halves(left, right);
        rucksack_items.push(common_item?);
    }

    Ok(rucksack_items)
}

fn get_rucksack_items_part2(text: &str) -> Result<Vec<char>> {
    let mut rucksack_items: Vec<char> = Vec::new();
    let mut iterator = text.lines();
    loop {
        let first_option = iterator.next();
        let second_option = iterator.next();
        let third_option = iterator.next();
        match (first_option, second_option, third_option) {
            (Some(first), Some(second), Some(third)) => {
                let common_item: Result<char> =
                    get_common_item_in_3_rucksacks(first, second, third);
                rucksack_items.push(common_item?);
            }
            (_, _, _) => break,
        }
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

fn get_common_item_in_3_rucksacks(first: &str, second: &str, third: &str) -> Result<char> {
    for char_first in first.chars() {
        for char_second in second.chars() {
            if char_first.eq(&char_second) {
                for char_third in third.chars() {
                    if char_first.eq(&char_third) {
                        return Ok(char_first);
                    }
                }
            }
        }
    }

    Err(eyre!(
        "Not common character found in 3 rucksacks: '{}',  '{}' and '{}'",
        first,
        second,
        third,
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
    fn test_get_rucksack_items_part1() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
        let expected_result: Vec<char> = vec!['p', 'L', 'P', 'v', 't', 's'];

        let output_list = get_rucksack_items_part1(input_str).unwrap();

        assert_eq!(output_list.len(), expected_result.len());

        let matching = output_list
            .iter()
            .zip(&expected_result)
            .filter(|&(a, b)| a.eq(b))
            .count();

        assert_eq!(matching, expected_result.len());
    }

    #[rstest]
    #[case("abcd", "xxax", "dxxa", 'a')]
    fn test_get_common_item_in_3_rucksacks(
        #[case] first: &str,
        #[case] second: &str,
        #[case] third: &str,
        #[case] expected: char,
    ) {
        assert_eq!(
            get_common_item_in_3_rucksacks(first, second, third).unwrap(),
            expected
        );
    }

    #[test]
    fn test_get_rucksack_items_part2() {
        let input_str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
        let expected_result: Vec<char> = vec!['r', 'Z'];

        let output_list = get_rucksack_items_part2(input_str).unwrap();

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
