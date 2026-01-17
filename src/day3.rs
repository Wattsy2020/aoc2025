use std::cmp::max;
use std::num::ParseIntError;
use std::str::FromStr;

struct Bank {
    joltages: Vec<u8>,
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

#[allow(dead_code)]
pub fn solve_part1(contents: &str) -> u32 {
    parse_input(contents)
        .unwrap()
        .iter()
        .map(find_max_joltage)
        .sum()
}

// Solution planning:
// Another Greedy solution
// This time build the number starting from the lowest decimal place digits
// When encountering a new digit:
// go through the current number
// remove the digit with the highest place value that is less than the new digit
// Problem: case like 28(12digits...) and 828(12digits...), you shouldn't add the two in the first, but should in the second

// What if we have 12 iterations
// In the first iteration we find the highest digit to start the number,
// while still having twelve digits and maximising for num remaining digits
// (e.g. taking the first 8 if there are multiple 8s)
// Then in second iteration we find the highest digit to start the second part of the number
// Would be O(n*12) or O(nk) where k = 12
fn find_max_joltage_inner(remaining_batteries: &[u8], num_batteries_to_take: u8) -> u64 {
    if num_batteries_to_take == 0 {
        return 0;
    }

    // find the highest digit we can take while still leaving (num_batteries_to_take - 1) digits
    let max_digit_idx = remaining_batteries
        .iter()
        .take(remaining_batteries.len() - num_batteries_to_take as usize + 1)
        .enumerate()
        .fold(0, |max_idx, (idx, digit)| {
            if *digit > remaining_batteries[max_idx] {
                idx
            } else {
                max_idx
            }
        });

    let max_digit_value =
        10u64.pow(num_batteries_to_take as u32 - 1) * (remaining_batteries[max_digit_idx] as u64);
    max_digit_value
        + find_max_joltage_inner(
            &remaining_batteries[(max_digit_idx + 1)..],
            num_batteries_to_take - 1,
        )
}

fn find_max_joltage_part2(bank: &Bank) -> u64 {
    find_max_joltage_inner(&bank.joltages, 12)
}

pub fn solve_part2(contents: &str) -> u64 {
    parse_input(contents)
        .unwrap()
        .iter()
        .map(find_max_joltage_part2)
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

    #[test]
    fn test_find_max_joltage_part2() {
        assert_eq!(calc_result("987654321111111"), 987654321111);
        assert_eq!(calc_result("811111111111119"), 811111111119);
        assert_eq!(calc_result("234234234234278"), 434234234278);
        assert_eq!(calc_result("818181911112111"), 888911112111);

        fn calc_result(joltages: &str) -> u64 {
            find_max_joltage_part2(&parse_bank(joltages).unwrap())
        }
    }
}
