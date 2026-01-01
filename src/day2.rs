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
    num + (digit as i64 * 10i64.pow(decimal_place))
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
) -> Vec<i64> {
    let repetition_distance = start_decimal_place - (start_decimal_place / 2);

    // check if we reached the second repetition,
    // in which case this number is already repeated twice and thus is an invalid id
    if current_decimal_place + repetition_distance == start_decimal_place {
        return if range.in_range(current_num) {
            vec![current_num]
        } else {
            vec![]
        };
    }

    // try all possible numbers for this first digit
    (0..10)
        .map(|digit| {
            set_digit(
                set_digit(current_num, current_decimal_place, digit), // set the current place
                current_decimal_place - repetition_distance, // then also set the digit that needs to repeat this digit
                digit,
            )
        })
        .filter(|num|
            // if the number is already greater than max, it will only get larger, thus isn't a solution
            *num <= range.end
            // if the number is less than the min, even if the remaining places were set to 99999..., then it can't be a solution
            && num + 10i64.pow(current_decimal_place) - 1 >= range.start)
        .flat_map(|num| {
            find_invalid_ids_from_number(range, num, current_decimal_place - 1, start_decimal_place)
        })
        .collect()
}

fn find_invalid_ids(range: &Range) -> Vec<i64> {
    // find decimal place of start and end digit
    // split up the search to within multiple ranges that each have the same decimal place
    // (skip odd numbered decimal places)
    // this simplifies the problem for `find_invalid_ids_from_number`,
    // it only has to consider numbers with the same number of decimal places
    let num_start_digits = num_digits(range.start);
    let num_end_digits = num_digits(range.end);
    let ranges: Vec<Range> = (num_start_digits..=num_end_digits)
        .filter(|num_digits| *num_digits > 0u32 && num_digits % 2 == 0)
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
        .collect();

    ranges
        .iter()
        .flat_map(|range| {
            let start_decimal_place = num_digits(range.start) - 1;
            find_invalid_ids_from_number(&range, 0, start_decimal_place, start_decimal_place)
        })
        .collect()
}

pub fn solve_part1(input: &str) -> i64 {
    parse_input(input).iter().flat_map(find_invalid_ids).sum()
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
    fn test_find_invalid_ids_for_number() {
        assert_eq!(
            find_invalid_ids_from_number(&Range { start: 10, end: 99 }, 0, 1, 1),
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
                3
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
}
