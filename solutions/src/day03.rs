extern crate substring;

use std::collections::HashSet;
use substring::Substring;

fn char_set(s: &str) -> HashSet<char> {
  HashSet::from_iter(s.chars())
}

fn char_val(c: char) -> i32 {
  match c {
    'a'..='z' => (c as i32) - ('a' as i32) + 1,
    'A'..='Z' => (c as i32) - ('A' as i32) + 27,
    _ => 0,
  }
}

fn part1(lines: &Vec<String>) -> i32 {
  let mut sum = 0;
  for s in lines {
    let n = s.len();
    let a = char_set(s.substring(0, n / 2));
    let b = char_set(s.substring(n / 2, n));
    sum += char_val(*(&a & &b).iter().next().unwrap());
  }
  sum
}

fn part2(lines: &Vec<String>) -> i32 {
  let mut sum = 0;
  for i in (0..lines.len()).step_by(3) {
    let a = char_set(&lines[i]);
    let b = char_set(&lines[i + 1]);
    let c = char_set(&lines[i + 2]);
    sum += char_val(*(&(&a & &b) & &c).iter().next().unwrap());
  }
  sum
}

pub fn doit(lines: &Vec<String>) -> (Option<String>, Option<String>) {
  (Some(part1(lines).to_string()), Some(part2(lines).to_string()))
}

extern crate macros;
macros::make_test!(day03, 157, 70);
