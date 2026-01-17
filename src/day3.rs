use std::cmp::max;
use std::num::ParseIntError;
use std::str::FromStr;

struct Bank {
    joltages: Vec<u8>
}

fn parse_bank(line: &str) -> Result<Bank, ParseIntError> {
    let joltages: Result<Vec<u8>, ParseIntError> = line
        .chars()
        .map(|char| u8::from_str(&char.to_string()))
        .collect();
    joltages.map(|joltages| Bank { joltages })
}

fn parse_input(contents: &str) -> Result<Vec<Bank>, ParseIntError> {
    contents.lines().map(parse_bank).collect()
}

/// Greedy solution.
/// Tracks the largest number seen previously, and the largest combination so far
/// then replaces the largest combination if we encounter a new largest combination
fn find_max_joltage(bank: &Bank) -> u32 {
    let mut max_starter = 0;
    let mut max_combination = 0u32;
    for joltage in bank.joltages.iter() {
        let new_combination = (max_starter as u32) * 10 + (*joltage as u32);
        max_combination = max(max_combination, new_combination);
        max_starter = max(max_starter, *joltage);
    }
    max_combination
}

pub fn solve_part1(contents: &str) -> u32 {
    parse_input(contents)
        .unwrap()
        .iter()
        .map(find_max_joltage)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_joltage() {
        assert_eq!(calc_result("987654321111111"), 98);
        assert_eq!(calc_result("811111111111119"), 89);
        assert_eq!(calc_result("234234234234278"), 78);
        assert_eq!(calc_result("818181911112111"), 92);

        fn calc_result(joltages: &str) -> u32 {
            find_max_joltage(&parse_bank(joltages).unwrap())
        }
    }
}
