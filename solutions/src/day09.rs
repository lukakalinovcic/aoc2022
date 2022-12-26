use std::collections::HashSet;

fn solve(lines: &Vec<String>, n: usize) -> usize {
  let mut snake: Vec<(i32, i32)> = vec![(0, 0); n];
  let mut seen: HashSet<(i32, i32)> = HashSet::new();
  for line in lines {
    let parts: Vec<&str> = line.split(" ").collect();
    let k = parts[1].parse::<i32>().unwrap();
    for _ in 0..k {
      match parts[0] {
        "D" => snake[0].0 -= 1,
        "U" => snake[0].0 += 1,
        "L" => snake[0].1 -= 1,
        "R" => snake[0].1 += 1,
        _ => (),
      }
      for i in 1..n {
        let d0 = snake[i - 1].0 - snake[i].0;
        let d1 = snake[i - 1].1 - snake[i].1;
        if d0.abs() >= 2 || d1.abs() >= 2 {
          snake[i].0 += d0.signum();
          snake[i].1 += d1.signum();
        }
      }
      seen.insert(snake[n - 1]);
    }
  }
  seen.len()
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  (solve(&lines, 2).to_string(), solve(&lines, 10).to_string())
}

extern crate macros;
macros::tests!(
  day09_1, 13, 1;
  day09_2, 88, 36;
);
