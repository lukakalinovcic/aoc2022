pub fn doit(lines: &Vec<String>) -> (Option<String>, Option<String>) {
  let (rows, columns) = (lines.len() as i32, lines[0].len() as i32);
  let cell = |r, c| lines[r as usize].as_bytes()[c as usize];
  let is_inside = |r, c| r >= 0 && r < rows && c >= 0 && c < columns;

  let mut num_visible = 0;
  let mut best_score = 0;
  for r0 in 0..rows {
    for c0 in 0..columns {
      let mut visible = false;
      let mut score = 1;
      for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (mut r, mut c) = (r0 + dr, c0 + dc);
        let mut dist = 1;
        while is_inside(r, c) && cell(r, c) < cell(r0, c0) {
          (r, c) = (r + dr, c + dc);
          dist += 1
        }
        if !is_inside(r, c) {
          visible = true;
          dist -= 1;
        }
        score *= dist;
      }
      if visible {
        num_visible += 1;
      }
      best_score = std::cmp::max(best_score, score);
    }
  }
  (Some(num_visible.to_string()), Some(best_score.to_string()))
}

extern crate macros;
macros::make_test!(day08, 21, 8);
