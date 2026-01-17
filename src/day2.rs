use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn in_range(&self, num: i64) -> bool {
        self.start <= num && num <= self.end
    }
}

fn parse_range(range: &str) -> Range {
    let range_parts: Vec<i64> = range
        .split('-')
        .map(|str| i64::from_str(str).unwrap())
        .collect();
    Range {
        start: range_parts[0],
        end: range_parts[1],
    }
}

fn parse_input(input: &str) -> Vec<Range> {
    input.split(',').map(parse_range).collect()
}

/// Return a number with the digit at place value `decimal_place` in `num` set to `digit`
/// Assumes the digit was zero previously, and that `digit` is in [0, 9]
fn set_digit(num: i64, decimal_place: u32, digit: u8) -> i64 {
    assert!(digit <= 9);
    num + (digit as i64 * 10i64.pow(decimal_place))
}

/// Return a number with the digits at place value `decimal_place` - `n`*`repetition_distance` in `num` set to `digit`
/// Assumes the digit was zero previously, that `digit` is in [0, 9]
fn set_digit_repeating(num: i64, decimal_place: u32, repetition_distance: u32, digit: u8) -> i64 {
    let mut result = num;
    let mut current_place = decimal_place;
    loop {
        result = set_digit(result, current_place, digit);
        if current_place < repetition_distance {
            break;
        }
        current_place -= repetition_distance;
    }
    result
}

/// Return the number of digits in `num`
fn num_digits(num: i64) -> u32 {
    let mut num = num;
    let mut num_digits = 0;
    while num != 0 {
        num /= 10;
        num_digits += 1;
    }
    num_digits
}

/// Find invalid ids that can be formed from the `current_number`
/// by adding digits at and after the `current_decimal_place`
///
/// Finds ids with a sequence of digits repeated twice using a depth first search:
/// 1. Try all possible first digits (that fit in the range, e.g. check that 1010 is in range, 2020 and so on)
/// 2. Then recurse and try all the possibilities for the next digit that stay in the range
fn find_invalid_ids_from_number(
    range: &Range,
    current_num: i64,
    current_decimal_place: u32,
    start_decimal_place: u32,
    repetition_distance: u32,
) -> Vec<i64> {
    // try all possible numbers for this first digit
    (0..10)
        .map(|digit| {
            set_digit_repeating(
                current_num,
                current_decimal_place,
                repetition_distance,
                digit,
            )
        })
        .filter(|num|
            // if the number is already greater than max, it will only get larger, thus isn't a solution
            *num <= range.end
            // if the number is less than the min, even if the remaining places were set to 99999..., then it can't be a solution
            && num + 10i64.pow(current_decimal_place) > range.start)
        .flat_map(|num| {
            // check if the next iteration reaches the numbers that have already been repeated,
            // in which case this number's digits are already completely filled, and thus can check if it's an invalid id
            if current_decimal_place + repetition_distance == start_decimal_place + 1 {
                return if range.in_range(num) {
                    vec![num]
                } else {
                    vec![]
                };
            }

            find_invalid_ids_from_number(
                range,
                num,
                current_decimal_place - 1,
                start_decimal_place,
                repetition_distance,
            )
        })
        .collect()
}

/// Split up the range to multiple ranges that each have the same decimal place.
/// This simplifies the problem for `find_invalid_ids_from_number`,
/// it only has to consider numbers with the same number of decimal places
fn get_decimal_place_aligned_ranges(range: &Range) -> Vec<Range> {
    let num_start_digits = num_digits(range.start);
    let num_end_digits = num_digits(range.end);
    (num_start_digits..=num_end_digits)
        .map(|num_digits| Range {
            start: if num_digits == num_start_digits {
                range.start
            } else {
                10i64.pow(num_digits - 1)
            },
            end: if num_digits == num_end_digits {
                range.end
            } else {
                10i64.pow(num_digits) - 1
            },
        })
        .collect()
}

fn find_invalid_ids(range: &Range) -> Vec<i64> {
    get_decimal_place_aligned_ranges(range)
        .into_iter()
        .flat_map(|range| {
            let num_digits = num_digits(range.start);
            if num_digits == 0u32 || num_digits % 2 != 0 {
                return vec![];
            }

            let start_decimal_place = num_digits - 1;
            let repetition_distance = num_digits / 2; // for part1 repeat numbers twice
            find_invalid_ids_from_number(
                &range,
                0,
                start_decimal_place,
                start_decimal_place,
                repetition_distance,
            )
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> i64 {
    parse_input(input).iter().flat_map(find_invalid_ids).sum()
}

/// Calculate factors of a number
fn factors(num: i64) -> Vec<i64> {
    let mut factors = vec![];
    for i in 2..=num.isqrt() {
        if num % i == 0 {
            factors.push(i);
            factors.push(num / i);
        }
    }
    factors
}

// part2: same general principle but we have to generalise find_invalid_ids
// to take a `num_repetitions` parameter, then call it with all possible repetitions for that number of decimal places
fn find_invalid_ids_part2(range: &Range) -> Vec<i64> {
    get_decimal_place_aligned_ranges(range)
        .into_iter()
        .flat_map(|range| {
            let num_digits = num_digits(range.start);
            if num_digits <= 1 {
                return HashSet::new();
            }
            let start_decimal_place = num_digits - 1;

            // we can repeat numbers using a repetition that evenly divides the number of digits in the number
            let mut repetition_distances = factors(num_digits as i64);
            repetition_distances.push(1);

            repetition_distances
                .iter()
                .flat_map(|repetition_distance| {
                    find_invalid_ids_from_number(
                        &range,
                        0,
                        start_decimal_place,
                        start_decimal_place,
                        *repetition_distance as u32,
                    )
                })
                // use HashSet to remove duplicate results
                // (e.g. 1111 is 11 repeating twice, and 1 repeating four times)
                .collect::<HashSet<i64>>()
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve_part2(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .flat_map(find_invalid_ids_part2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 0);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(99), 2);
        assert_eq!(num_digits(09), 1);
        assert_eq!(num_digits(1230645), 7);
    }

    #[test]
    fn test_set_digit() {
        assert_eq!(set_digit(0, 0, 3), 3);
        assert_eq!(set_digit(0, 2, 5), 500);
        assert_eq!(set_digit(1230567, 3, 9), 1239567);
    }

    #[test]
    fn test_set_digit_repeating() {
        assert_eq!(set_digit_repeating(0, 3, 1, 3), 3333);
        assert_eq!(set_digit_repeating(0, 3, 2, 3), 3030);
        assert_eq!(set_digit_repeating(0, 3, 3, 3), 3003);
        assert_eq!(set_digit_repeating(0, 3, 4, 3), 3000);
        assert_eq!(set_digit_repeating(0, 3, 5, 3), 3000);
        assert_eq!(set_digit_repeating(0, 7, 3, 5), 50050050);
    }

    #[test]
    fn test_find_invalid_ids_for_number() {
        assert_eq!(
            find_invalid_ids_from_number(&Range { start: 10, end: 99 }, 0, 1, 1, 1),
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99]
        );
        assert_eq!(
            find_invalid_ids_from_number(
                &Range {
                    start: 1000,
                    end: 1300
                },
                0,
                3,
                3,
                2
            ),
            vec![1010, 1111, 1212]
        );
    }

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(
            find_invalid_ids(&Range {
                start: 1188511880,
                end: 1188511890
            }),
            vec![1188511885]
        );
        assert_eq!(
            find_invalid_ids(&Range {
                start: 0,
                end: 2300
            }),
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212, 1313, 1414, 1515, 1616, 1717,
                1818, 1919, 2020, 2121, 2222
            ]
        )
    }

    #[test]
    fn test_find_invalid_ids_part2() {
        let mut results = find_invalid_ids_part2(&Range {
            start: 0,
            end: 2300,
        });
        results.sort();
        assert_eq!(
            results,
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222, 333, 444, 555, 666, 777, 888, 999,
                1010, 1111, 1212, 1313, 1414, 1515, 1616, 1717, 1818, 1919, 2020, 2121, 2222
            ]
        )
    }
}
