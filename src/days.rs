pub mod day01;
pub mod day02;
pub mod day03;

pub fn run_day(day: u8, input: &str) {
    match day {
        1 => day01::run(input),
        2 => day02::run(input),
        3 => day03::run(input),
        _ => eprintln!("Day {} not implemented yet.", day),
    }
}
