use multiarray::*;

fn read_input(lines: &Vec<String>) -> (Vec<Vec<u8>>, usize, usize) {
  let rows = lines.len() - 2;
  let cols = lines[0].len() - 2;
  (
    lines.iter().map(|line| (*line).bytes().collect()).collect(),
    rows,
    cols,
  )
}

fn gcd<T: num::Integer + num::Zero + Copy>(x: T, y: T) -> T {
  match y {
    z if z == num::zero() => x,
    _ => gcd(y, x % y),
  }
}

fn lcm<T: num::Integer + num::Zero + Copy>(x: T, y: T) -> T {
  x / gcd(x, y) * y
}

fn init_blizzards(map: &Vec<Vec<u8>>, rows: usize, cols: usize) -> (Array4D<bool>, usize) {
  let cycle = lcm(rows, cols);
  let mut seen = Array4D::new([rows + 2, cols + 2, cycle, 3], false);
  for r in 1..=rows {
    for c in 1..=cols {
      if map[r][c] == b'.' {
        continue;
      }
      for k in 0..cycle {
        for i in 0..3 {
          match map[r][c] {
            b'>' => seen[[r, 1 + (c - 1 + k) % cols, k, i]] = true,
            b'<' => seen[[r, 1 + (c - 1 + cycle - k) % cols, k, i]] = true,
            b'v' => seen[[1 + (r - 1 + k) % rows, c, k, i]] = true,
            b'^' => seen[[1 + (r - 1 + cycle - k) % rows, c, k, i]] = true,
            _ => panic!("unexpected character"),
          }
        }
      }
    }
  }
  (seen, cycle)
}

type Pt = (i8, i8);

struct QueueItem {
  pos: Pt,
  t: i16,
  stage: i8,
  dist: i16,
}

impl QueueItem {
  fn new(pos: Pt, t: i16, stage: i8, dist: i16) -> Self {
    Self {
      pos,
      t,
      stage,
      dist,
    }
  }
}

fn solve(mut seen: Array4D<bool>, rows: usize, cols: usize, cycle: usize) -> (i16, i16) {
  let (rows, cols) = (rows as i8, cols as i8);
  let start: Pt = (0, 1);
  let end: Pt = (rows + 1, cols);
  let is_inside =
    |p: Pt| (p.0 >= 1 && p.0 <= rows && p.1 >= 1 && p.1 <= cols) || p == start || p == end;

  let mut queue = std::collections::VecDeque::from([QueueItem::new(start, 0, 0, 0)]);
  let mut part1 = -1;
  while !queue.is_empty() {
    let item = queue.pop_front().unwrap();
    let mut stage = item.stage as usize;
    if item.pos == end && stage % 2 == 0 {
      if stage == 0 && part1 == -1 {
        part1 = item.dist;
      }
      if stage == 2 {
        return (part1, item.dist);
      }
      stage += 1;
    }
    if item.pos == start && stage == 1 {
      stage += 1;
    }

    let new_t = (item.t as usize + 1) % cycle;

    const DELTAS: [Pt; 5] = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];
    for delta in DELTAS {
      let new_pos = (item.pos.0 + delta.0, item.pos.1 + delta.1);
      let (r, c) = (new_pos.0 as usize, new_pos.1 as usize);
      if is_inside(new_pos) && !seen[[r, c, new_t, stage]] {
        seen[[r, c, new_t, stage]] = true;
        queue.push_back(QueueItem::new(
          new_pos,
          new_t as i16,
          stage as i8,
          item.dist + 1,
        ));
      }
    }
  }
  panic!("unreachable")
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let (map, rows, cols) = read_input(lines);
  let (seen, cycle) = init_blizzards(&map, rows, cols);
  let (part1, part2) = solve(seen, rows, cols, cycle);
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day24, 18, 54;);
