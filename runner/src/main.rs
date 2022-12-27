extern crate solutions;

use std::io::Write;

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
    let start = std::time::Instant::now();

    print!("Day #{day:02} ... ");
    std::io::stdout().flush().unwrap();

    let (part1, part2) = solutions::run(day);

    let duration = start.elapsed().as_secs_f32();
    println!("(done in {duration:.2}s) => {part1}, {part2}");
  }
}
