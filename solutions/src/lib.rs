extern crate macros;
macros::register_solutions!(2);

fn read_lines(day: usize) -> Vec<String> {
  use std::fs::File;
  use std::io::{self, BufRead};
  let filename = format!("inputs/day{:02}.in", day);
  let file = File::open(filename).unwrap();
  io::BufReader::new(file)
    .lines()
    .map(|line| line.unwrap())
    .collect()
}

pub fn run(day: usize) -> (Option<String>, Option<String>) {
  REGISTRY[day - 1](&read_lines(day))
}
