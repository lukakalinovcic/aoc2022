#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Rock {
  x: i32,
  y: i32,
}

type Rocks = std::collections::BTreeSet<Rock>;

impl Rock {
  fn new(x: i32, y: i32) -> Rock {
    Rock { x, y }
  }

  fn from(s: &str) -> Option<Rock> {
    let (x_str, y_str) = s.split_once(",")?;
    Some(Rock::new(x_str.parse().ok()?, y_str.parse().ok()?))
  }

  fn move_towards(self, other: &Rock) -> Rock {
    let dx = (other.x - self.x).signum();
    let dy = (other.y - self.y).signum();
    Rock::new(self.x + dx, self.y + dy)
  }

  fn drop(self, rocks: &Rocks, floor: i32) -> Rock {
    let floor = Rock::new(self.x, floor);
    let q = rocks
      .range((
        std::ops::Bound::Included(&self),
        std::ops::Bound::Excluded(&floor),
      ))
      .next()
      .unwrap_or(&floor);
    Rock::new(q.x, q.y - 1)
  }

  fn slip(self, rocks: &Rocks, floor: i32) -> (Rock, bool) {
    let lt = Rock::new(self.x - 1, self.y + 1);
    if lt.y < floor && !rocks.contains(&lt) {
      return (lt, true);
    }
    let rt = Rock::new(self.x + 1, self.y + 1);
    if rt.y < floor && !rocks.contains(&rt) {
      return (rt, true);
    }
    return (self, false);
  }
}

fn process_input(lines: &Vec<String>) -> Rocks {
  let mut rocks = Rocks::new();
  for line in lines {
    let pts: Vec<_> = line.split(" -> ").map(|s| Rock::from(s).unwrap()).collect();
    for (start, end) in pts.iter().zip(pts.iter().skip(1)) {
      let mut p = (*start).clone();
      while p != *end {
        rocks.insert(p.clone());
        p = p.move_towards(end);
      }
    }
    rocks.insert((*pts.last().unwrap()).clone());
  }
  rocks
}

fn drop_rocks(rocks: &mut Rocks, floor: i32, done: impl Fn(&Rock) -> bool) -> i32 {
  for i in 0.. {
    let mut p = Rock::new(500, 0);
    loop {
      p = p.drop(rocks, floor);
      if done(&p) {
        return i;
      }
      let moved;
      (p, moved) = p.slip(rocks, floor);
      if !moved {
        break;
      }
    }
    rocks.insert(p);
  }
  panic!("unreachable");
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let mut rocks = process_input(lines);
  let floor = rocks.iter().map(|p| p.y).max().unwrap() + 2;
  let p1 = drop_rocks(&mut rocks, floor, |p| p.y == floor - 1);
  let p2 = drop_rocks(&mut rocks, floor, |p| p.y == -1);
  (p1.to_string(), (p1 + p2).to_string())
}

extern crate macros;
macros::tests!(day14, 24, 93;);
