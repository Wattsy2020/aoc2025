use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    magnitude: u32,
}

fn parse_instruction(instruction: &str) -> Instruction {
    let chars: Box<[char]> = instruction.chars().collect();
    let direction = match chars[0] {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Unexpected direction {}", chars[0]),
    };
    let magnitude: String = chars[1..].iter().collect();
    Instruction {
        direction,
        magnitude: u32::from_str(&magnitude).unwrap(),
    }
}

fn parse_instructions(instructions: &str) -> Box<[Instruction]> {
    instructions
        .split_ascii_whitespace()
        .map(parse_instruction)
        .collect()
}

#[allow(dead_code)]
pub fn solve(problem_input: &str) -> u32 {
    let instructions = parse_instructions(problem_input);

    let mut num_times_at_zero = 0u32;
    let mut current_position = 50;
    for Instruction {
        direction,
        magnitude,
    } in instructions
    {
        match direction {
            Direction::Left => {
                current_position = (current_position - magnitude as i32).rem_euclid(100)
            }
            Direction::Right => {
                current_position = (current_position + magnitude as i32).rem_euclid(100)
            }
        }
        if current_position == 0 {
            num_times_at_zero += 1
        }
    }
    num_times_at_zero
}

// when rotating right: (current pos + magnitude) / 100 (integer division) gives the number of rotations past 100
// when rotating left: (100 - current pos + magnitude) / 100
pub fn solve_part2(problem_input: &str) -> u32 {
    let instructions = parse_instructions(problem_input);

    let mut num_times_at_zero = 0u32;
    let mut current_position = 50;
    for Instruction {
        direction,
        magnitude,
    } in instructions
    {
        println!("{direction:?} {magnitude} {current_position} {num_times_at_zero}");
        match direction {
            Direction::Left => {
                let num_steps_to_zero = if current_position == 0 {
                    100
                } else {
                    current_position
                };
                let num_extra_times_at_zero: u32 = ((100 - num_steps_to_zero + magnitude as i32)
                    / 100)
                    .try_into()
                    .expect("should be positive");
                current_position = (current_position - magnitude as i32).rem_euclid(100);
                num_times_at_zero += num_extra_times_at_zero;
            }
            Direction::Right => {
                let num_extra_times_at_zero: u32 = ((current_position + magnitude as i32) / 100)
                    .try_into()
                    .expect("should be positive");
                current_position = (current_position + magnitude as i32).rem_euclid(100);
                num_times_at_zero += num_extra_times_at_zero;
            }
        }
    }
    num_times_at_zero
}

// 6823 is too high
