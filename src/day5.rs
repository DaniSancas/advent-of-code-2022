use aoc_runner_derive::aoc;
use eyre::{eyre, Result};
use regex::Regex;
use std::collections::VecDeque;
use std::string::ToString;

const REGEX: &str = r"move (\d+) from (\d+) to (\d+)";

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let result = get_crates_number_of_stacks_and_moves(input)
        .and_then(|(crates, number_of_stacks, moves)| fill_stacks(crates, number_of_stacks, moves))
        .and_then(|(list_of_stacks, moves)| follow_moving_instructions_part1(list_of_stacks, moves))
        .and_then(get_last_crate_of_each_stack);

    match result {
        Ok(r) => r,
        Err(e) => e.to_string(),
    }
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let result = get_crates_number_of_stacks_and_moves(input)
        .and_then(|(crates, number_of_stacks, moves)| fill_stacks(crates, number_of_stacks, moves))
        .and_then(|(list_of_stacks, moves)| follow_moving_instructions_part2(list_of_stacks, moves))
        .and_then(get_last_crate_of_each_stack);

    match result {
        Ok(r) => r,
        Err(e) => e.to_string(),
    }
}

fn get_crates_number_of_stacks_and_moves(input: &str) -> Result<(&str, u32, &str)> {
    input
        .split_once("\n\n")
        .ok_or_else(|| eyre!("Couldn't get crates, number of stacks and moves."))
        .and_then(|(crates_and_numbers, moves)| {
            let number_list = crates_and_numbers
                .lines()
                .last()
                .ok_or_else(|| eyre!("Couldn't get list of number of stacks"))?;

            let (crates, _) = crates_and_numbers
                .split_once(number_list)
                .ok_or_else(|| eyre!("Couldn't get crates without list of number of stacks."))?;

            let number: u32 = number_list
                .chars()
                .nth_back(1)
                .ok_or_else(|| eyre!("Couldn't get the max number of stacks."))?
                .to_digit(10)
                .ok_or_else(|| eyre!("Couldn't convert the max number of stacks."))?;

            Ok((crates, number, moves))
        })
}

fn fill_stacks<'a>(
    crates: &str,
    number_of_stacks: u32,
    moves: &'a str,
) -> Result<(Vec<VecDeque<char>>, &'a str)> {
    let mut list_of_stacks: Vec<VecDeque<char>> = Vec::new();
    for stack_number in 0..number_of_stacks {
        let mut stack: VecDeque<char> = VecDeque::new();
        for line in crates.lines() {
            let position: usize = (stack_number * 4) as usize + 1;
            match line.chars().into_iter().nth(position) {
                Some(c) if c.ne(&' ') => stack.push_front(c),
                _ => (),
            }
        }
        list_of_stacks.push(stack);
    }

    Ok((list_of_stacks, moves))
}

fn follow_moving_instructions_part1(
    mut list_of_stacks: Vec<VecDeque<char>>,
    moves: &str,
) -> Result<Vec<VecDeque<char>>> {
    let regex = Regex::new(REGEX)?;
    for line in moves.lines() {
        let (number_of_movements, origin, destination) = parse_instructions(line, &regex)?;
        for _ in 0..number_of_movements {
            let origin_stack: &mut VecDeque<char> =
                list_of_stacks
                    .as_mut_slice()
                    .get_mut(origin)
                    .ok_or_else(|| eyre!("Couldn't get origin stack."))?;

            let moved_crate = origin_stack
                .pop_back()
                .ok_or_else(|| eyre!("Couldn't pop crate from origin stack."))?;

            let destination_stack: &mut VecDeque<char> = list_of_stacks
                .as_mut_slice()
                .get_mut(destination)
                .ok_or_else(|| eyre!("Couldn't get destination stack."))?;

            destination_stack.push_back(moved_crate);
        }
    }

    Ok(list_of_stacks)
}

fn follow_moving_instructions_part2(
    mut list_of_stacks: Vec<VecDeque<char>>,
    moves: &str,
) -> Result<Vec<VecDeque<char>>> {
    let regex = Regex::new(REGEX)?;
    for line in moves.lines() {
        let (number_of_movements, origin, destination) = parse_instructions(line, &regex)?;

        let origin_stack: &mut VecDeque<char> = list_of_stacks
            .as_mut_slice()
            .get_mut(origin)
            .ok_or_else(|| eyre!("Couldn't get origin stack."))?;

        let mut crate_movement_queue: VecDeque<char> = VecDeque::new();

        for _ in 0..number_of_movements {
            let moved_crate = origin_stack
                .pop_back()
                .ok_or_else(|| eyre!("Couldn't pop crate from origin stack."))?;
            crate_movement_queue.push_back(moved_crate);
        }

        let destination_stack: &mut VecDeque<char> = list_of_stacks
            .as_mut_slice()
            .get_mut(destination)
            .ok_or_else(|| eyre!("Couldn't get destination stack."))?;

        for _ in 0..number_of_movements {
            let moved_crate = crate_movement_queue
                .pop_back()
                .ok_or_else(|| eyre!("Couldn't pop crate from intermediate stack."))?;

            destination_stack.push_back(moved_crate);
        }
    }

    Ok(list_of_stacks)
}

fn parse_instructions(line: &str, regex: &Regex) -> Result<(u32, usize, usize)> {
    let groups = regex
        .captures(line)
        .ok_or_else(|| eyre!("Couldn't parse instruction groups."))?;

    let number_of_movements = groups[1].parse::<u32>()?;
    let origin = groups[2].parse::<usize>()?;
    let destination = groups[3].parse::<usize>()?;

    Ok((number_of_movements, origin - 1, destination - 1))
}

fn get_last_crate_of_each_stack(list_of_stacks: Vec<VecDeque<char>>) -> Result<String> {
    let last_crates: String = list_of_stacks
        .into_iter()
        .map(|stack| stack.back().map_or_else(String::new, ToString::to_string))
        .collect();

    Ok(last_crates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_get_crates_number_of_stacks_and_moves() {
        let crates = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n";
        let numbers: &str = "     1   2   3 \n";
        let number: u32 = 3;
        let moves: &str =
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

        let input = format!("{}{}\n{}", crates, numbers, moves);

        let output = get_crates_number_of_stacks_and_moves(&input).unwrap();

        assert_eq!(output.0, crates);
        assert_eq!(output.1, number);
        assert_eq!(output.2, moves);
    }

    #[test]
    fn test_fill_stacks() {
        let crates = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n";
        let number_of_stacks: u32 = 3;
        let moves: &str =
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

        let output = fill_stacks(crates, number_of_stacks, moves).unwrap();
        let list_of_stacks = output.0;

        assert_eq!(list_of_stacks[0], VecDeque::from(['Z', 'N']));
        assert_eq!(list_of_stacks[1], VecDeque::from(['M', 'C', 'D']));
        assert_eq!(list_of_stacks[2], VecDeque::from(['P']));
        assert_eq!(output.1, moves);
    }

    #[rstest]
    #[case("move 1 from 2 to 1", (1, 1, 0))]
    #[case("move 3 from 1 to 3", (3, 0, 2))]
    #[case("move 2 from 2 to 1", (2, 1, 0))]
    #[case("move 1 from 1 to 2", (1, 0, 1))]
    #[case("move 31 from 7 to 1", (31, 6, 0))]
    fn test_parse_instructions(#[case] line: &str, #[case] expected: (u32, usize, usize)) {
        let regex = Regex::new(REGEX).unwrap();
        let output = parse_instructions(line, &regex).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_follow_moving_instructions_part1() {
        let list_of_stacks = vec![
            VecDeque::from(['Z', 'N']),
            VecDeque::from(['M', 'C', 'D']),
            VecDeque::from(['P']),
        ];
        let moves: &str =
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

        let output = follow_moving_instructions_part1(list_of_stacks, moves).unwrap();

        assert_eq!(output[0], VecDeque::from(['C']));
        assert_eq!(output[1], VecDeque::from(['M']));
        assert_eq!(output[2], VecDeque::from(['P', 'D', 'N', 'Z']));
    }

    #[test]
    fn test_follow_moving_instructions_part2() {
        let list_of_stacks = vec![
            VecDeque::from(['Z', 'N']),
            VecDeque::from(['M', 'C', 'D']),
            VecDeque::from(['P']),
        ];
        let moves: &str =
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

        let output = follow_moving_instructions_part2(list_of_stacks, moves).unwrap();

        assert_eq!(output[0], VecDeque::from(['M']));
        assert_eq!(output[1], VecDeque::from(['C']));
        assert_eq!(output[2], VecDeque::from(['P', 'Z', 'N', 'D']));
    }
    #[test]
    fn test_get_last_crate_of_each_stack() {
        let list_of_stacks: Vec<VecDeque<char>> = vec![
            VecDeque::from(['Z', 'N']),
            VecDeque::from(['M', 'C', 'D']),
            VecDeque::from(['P']),
        ];

        let expected = "NDP".to_string();

        let output = get_last_crate_of_each_stack(list_of_stacks).unwrap();

        assert_eq!(output, expected);
    }
}
