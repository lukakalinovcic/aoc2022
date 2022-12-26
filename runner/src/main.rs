extern crate solutions;

use clap::Parser;

/// Simple binary to run Advent of code 2022 solutions.
#[derive(Parser, Debug)]
struct Args {
   /// Days to run.
   #[arg(short, long)]
   days: Vec<usize>,
}

fn main() {
  let args = Args::parse();
  for day in args.days {
    println!("Day{}: {:?}", day, solutions::run(day));
  }
}