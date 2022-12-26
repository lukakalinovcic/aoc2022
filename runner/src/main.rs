extern crate solutions;

use clap::Parser;

/// Simple binary to run Advent of code 2022 solutions.
#[derive(Parser, Debug)]
struct Args {
   /// Days to run. Leave empty to run all days.
   #[arg(short, long)]
   days: Vec<usize>,
}

fn main() {
  let mut days = Args::parse().days;
  if days.is_empty() {
    days = (1..=solutions::SOLUTIONS).collect();
  }
  for day in days {
    println!("Day {}: {:?}", day, solutions::run(day));
  }
}