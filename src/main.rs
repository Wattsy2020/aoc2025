use std::fs;

mod day1;

fn read_day(day_no: u8) -> String {
    let filename = std::env::current_dir()
        .unwrap()
        .join("problem_inputs")
        .join(format!("day{}.txt", day_no));
    fs::read_to_string(filename).unwrap()
}

fn main() {
    println!("{}", day1::solve(&read_day(1)));
}
