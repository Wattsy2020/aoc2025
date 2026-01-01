use std::fs;

mod day1;
mod day2;

fn read_day(day_no: u8) -> String {
    let filename = std::env::current_dir()
        .unwrap()
        .join("problem_inputs")
        .join(format!("day{}.txt", day_no));
    fs::read_to_string(filename).unwrap()
}

fn main() {
    //println!("{}", day1::solve(&read_day(1)));
    //println!("{}", day1::solve_part2(&read_day(1)));
    println!("{}", day2::solve_part1(&read_day(2)));
}
