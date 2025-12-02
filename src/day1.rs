use std::str::FromStr;

enum Direction {
    Left,
    Right
}

struct Instruction {
    direction: Direction,
    magnitude: u32
}

fn parse_instruction(instruction: &str) -> Instruction {
    let chars: Box<[char]> = instruction.chars().collect();
    let direction = match chars[0] {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Unexpected direction {}", chars[0])
    };
    let magnitude: String = chars[1..].iter().collect();
    Instruction {
        direction,
        magnitude: u32::from_str(&magnitude).unwrap()
    }
}

fn parse_instructions(instructions: &str) -> Box<[Instruction]> {
    instructions
        .split_ascii_whitespace()
        .map(parse_instruction)
        .collect()
}

pub fn solve(problem_input: &str) -> u32 {
    let instructions = parse_instructions(problem_input);

    let mut num_times_at_zero = 0u32;
    let mut current_position = 50;
    for Instruction { direction, magnitude } in instructions {
        match direction {
            Direction::Left => current_position = (current_position - magnitude as i32).rem_euclid(100),
            Direction::Right => current_position = (current_position + magnitude as i32).rem_euclid(100)
        }
        if current_position == 0 {
            num_times_at_zero += 1
        }
    }
    num_times_at_zero
}