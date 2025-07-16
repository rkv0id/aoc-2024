mod days;
mod input;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Run Advent of Code 2024 solutions")]
struct Args {
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let input = input::fetch_input(2024, args.day);
    days::run_day(args.day, &input);
}
