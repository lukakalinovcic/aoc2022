use std::collections::HashSet;

fn solve(lines: &Vec<String>, k: usize) -> String {
  let chars: Vec<char> = lines[0].chars().collect();
  for i in k..=chars.len() {
    if HashSet::<&char>::from_iter(chars[i - k..i].iter()).len() == k {
      return i.to_string();
    }
  }
  "".to_string()
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  (solve(lines, 4), solve(lines, 14))
}

extern crate macros;
macros::tests!(
  day06_1, "7", "19";
  day06_2, "5", "23";
  day06_3, "6", "23";
  day06_4, "10", "29";
  day06_5, "11", "26";
);
