pub mod day01;

pub fn run_day(day: u8, input: &str) {
    match day {
        1 => day01::run(input),
        _ => eprintln!("Day {} not implemented yet.", day),
    }
}
