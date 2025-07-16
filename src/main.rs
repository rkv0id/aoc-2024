mod input;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aoc2024")]
#[command(author = "rkv0id")]
#[command(about = "Run Advent of Code 2024 solutions", long_about = None)]
struct Args {
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}

fn main() {
    let args = Args::parse();
    println!("Running solution for Day {}", args.day);

    let input = input::fetch_input(2024, args.day);
    println!("Fetched input for Day {}:\n{}", args.day, input);
}
