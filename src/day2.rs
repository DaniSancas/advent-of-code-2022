use aoc_runner_derive::aoc;
use eyre::Result;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total_score: u32 = 0;
    for line in input.lines() {
        let theirs = line.chars().nth(0);
        let mine = line.chars().nth(2);
        total_score += get_round_result(theirs, mine);
    }

    total_score
}

/// Resolves a round (left => them, right => me) and sums the score in two steps:
///
/// First step, points agains opponent:
/// - Win: 6
/// - Draw: 3
/// - Loss: 0
///
/// Second step, points for choosing an option:
/// - Rock: 1
/// - Paper: 2
/// - Scissors: 3
///
/// Example:
///
/// A vs Y results in a Win (6) + Paper selected (2) = 8 total points
fn get_round_result(theirs: Option<char>, mine: Option<char>) -> u32 {
    match (theirs, mine) {
        (Some(t), Some(m)) => match (t, m) {
            ('A', 'X') => 3 + 1, // Rock vs Rock
            ('A', 'Y') => 6 + 2, // Rock vs Paper
            ('A', 'Z') => 0 + 3, // Rock vs Scissors
            ('B', 'X') => 0 + 1, // Paper vs Rock
            ('B', 'Y') => 3 + 2, // Paper vs Paper
            ('B', 'Z') => 6 + 3, // Paper vs Scissors
            ('C', 'X') => 6 + 1, // Scissors vs Rock
            ('C', 'Y') => 0 + 2, // Scissors vs Paper
            ('C', 'Z') => 3 + 3, // Scissors vs Scissors
            (_, _) => 0,
        },
        (_, _) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(None, None, 0)]
    #[case(Some('A'), Some('Y'), 8)]
    #[case(Some('B'), Some('X'), 1)]
    #[case(Some('C'), Some('Z'), 6)]
    fn test_get_round_result(
        #[case] theirs: Option<char>,
        #[case] mine: Option<char>,
        #[case] expected: u32,
    ) {
        assert_eq!(get_round_result(theirs, mine), expected);
    }

    #[test]
    fn test_get_total_score() {
        let input_str = "A Y\nB X\nC Z\n";

        let expected_result: u32 = 15;

        assert_eq!(part1(input_str), expected_result);
    }
}
