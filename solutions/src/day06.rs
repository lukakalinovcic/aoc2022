use std::collections::HashSet;

fn solve(lines: &Vec<String>, k: usize) -> Option<String> {
  let mut results = Vec::new();
  for line in lines {
    let chars: Vec<char> = line.chars().collect();
    for i in k..=chars.len() {
      if HashSet::<&char>::from_iter(chars[i - k..i].iter()).len() == k {
        results.push(i.to_string());
        break;
      }
    }
  }
  Some(results.join(", "))
}

pub fn doit(lines: &Vec<String>) -> (Option<String>, Option<String>) {
  (solve(lines, 4), solve(lines, 14))
}

extern crate macros;
macros::make_test!(day06, "7, 5, 6, 10, 11", "19, 23, 23, 29, 26");
