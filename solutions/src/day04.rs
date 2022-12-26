use std::cmp;
use std::fmt::Debug;
use std::str::FromStr;

struct Range<T> {
  lo: T,
  hi: T,
}

impl<T: Ord + Copy> Range<T> {
  fn contains(&self, other: &Range<T>) -> bool {
    self.lo <= other.lo && self.hi >= other.hi
  }

  fn overlaps(&self, other: &Range<T>) -> bool {
    cmp::max(self.lo, other.lo) <= cmp::min(self.hi, other.hi)
  }
}

impl<T: FromStr + Debug> Range<T>
where
  <T as FromStr>::Err: Debug,
{
  fn new(s: &str) -> Range<T> {
    let parts: Vec<&str> = s.split("-").collect();
    Range {
      lo: parts[0].parse::<T>().unwrap(),
      hi: parts[1].parse::<T>().unwrap(),
    }
  }
}

fn solve(lines: &Vec<String>, predicate: impl Fn(&Range<i32>, &Range<i32>) -> bool) -> i32 {
  let mut cnt = 0;
  for line in lines {
    let parts: Vec<&str> = line.split(",").collect();
    let a = Range::<i32>::new(&parts[0]);
    let b = Range::<i32>::new(&parts[1]);
    if predicate(&a, &b) {
      cnt += 1;
    }
  }
  cnt
}

fn part1(lines: &Vec<String>) -> i32 {
  solve(lines, |a, b| a.contains(&b) || b.contains(&a))
}

fn part2(lines: &Vec<String>) -> i32 {
  solve(lines, |a, b| a.overlaps(&b))
}

pub fn doit(lines: &Vec<String>) -> (Option<String>, Option<String>) {
  (
    Some(part1(lines).to_string()),
    Some(part2(lines).to_string()),
  )
}

extern crate macros;
macros::make_test!(day04, 2, 4);
