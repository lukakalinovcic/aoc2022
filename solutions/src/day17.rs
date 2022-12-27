use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Pt(i32, i32);

struct Board {
  occupied: HashSet<Pt>,
  height: i32,
}

impl Board {
  fn new() -> Self {
    Board {
      occupied: HashSet::new(),
      height: 0,
    }
  }

  fn fits(&self, shape: &Vec<Pt>) -> bool {
    let ok = |p| !self.occupied.contains(p) && p.0 >= 1 && p.0 <= 7 && p.1 > 0;
    shape.iter().all(ok)
  }

  fn settle(&mut self, shape: &Vec<Pt>) {
    shape.iter().for_each(|p| {
      self.occupied.insert(*p);
      self.height = std::cmp::max(self.height, p.1);
    });
  }
}

fn make_shape(i: usize, y0: i32) -> Vec<Pt> {
  let (y1, y2, y3) = (y0 + 1, y0 + 2, y0 + 3);
  match i % 5 {
    0 => vec![Pt(3, y0), Pt(4, y0), Pt(5, y0), Pt(6, y0)],
    1 => vec![Pt(4, y0), Pt(3, y1), Pt(4, y1), Pt(5, y1), Pt(4, y2)],
    2 => vec![Pt(3, y0), Pt(4, y0), Pt(5, y0), Pt(5, y1), Pt(5, y2)],
    3 => vec![Pt(3, y0), Pt(3, y1), Pt(3, y2), Pt(3, y3)],
    4 => vec![Pt(3, y0), Pt(4, y0), Pt(3, y1), Pt(4, y1)],
    _ => vec![],
  }
}

struct Wind<'a> {
  i: usize,
  s: &'a str,
}

impl<'a> Wind<'a> {
  fn new(s: &'a str) -> Self {
    Wind { i: 0, s }
  }

  fn next(self: &mut Self) -> Pt {
    let p = match self.s.as_bytes()[self.i] {
      b'<' => Pt(-1, 0),
      b'>' => Pt(1, 0),
      _ => Pt(0, 0),
    };
    self.i = (self.i + 1) % self.s.len();
    p
  }
}

fn try_move(board: &Board, d: Pt, shape: Vec<Pt>) -> (Vec<Pt>, bool) {
  let new_shape: Vec<Pt> = shape.iter().map(|p| Pt(p.0 + d.0, p.1 + d.1)).collect();
  match board.fits(&new_shape) {
    false => (shape, false),
    true => (new_shape, true),
  }
}

fn drop(board: &mut Board, wind: &mut Wind, mut shape: Vec<Pt>) -> Vec<Pt> {
  loop {
    let ok;
    (shape, _) = try_move(board, wind.next(), shape);
    (shape, ok) = try_move(board, Pt(0, -1), shape);
    if !ok {
      return shape;
    }
  }
}

fn step(board: &mut Board, wind: &mut Wind, i: usize) {
  let shape = drop(board, wind, make_shape(i, board.height + 4));
  board.settle(&shape);
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let mut wind = Wind::new(lines[0].trim());
  let mut board = Board::new();
  for i in 0..2022 {
    step(&mut board, &mut wind, i);
  }
  let part1 = board.height;

  const TARGET: usize = 1000000000000;
  let mut i = 2022;
  let mut seen: HashMap<(usize, usize), (usize, i32)> = HashMap::new();
  let mut dh = 0;
  while i < TARGET {
    step(&mut board, &mut wind, i);
    i += 1;

    let h = board.height;
    let key = (i % 5, wind.i);
    if let Some((i0, h0)) = seen.get(&key) {
      let cycle_len = i - i0;
      let k = (TARGET - i) / cycle_len;
      i += k * cycle_len;
      dh += k * (h - h0) as usize;
    }
    seen.insert(key, (i, h));
  }
  let part2 = (board.height as usize) + dh;
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day17, 3068, 1514285714288;);
