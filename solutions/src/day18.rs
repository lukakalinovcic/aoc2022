type Pt = (i32, i32, i32);
type Pts = std::collections::HashSet<Pt>;

const DELTAS: [(i32, i32, i32); 6] = [
  (0, 0, -1),
  (0, 0, 1),
  (0, -1, 0),
  (0, 1, 0),
  (-1, 0, 0),
  (1, 0, 0),
];

fn get_trapped(pts: &Pts) -> Pts {
  let mut lo = (i32::MAX, i32::MAX, i32::MAX);
  let mut hi = (i32::MIN, i32::MIN, i32::MIN);
  for pt in pts {
    lo.0 = std::cmp::min(lo.0, pt.0 - 1);
    lo.1 = std::cmp::min(lo.1, pt.1 - 1);
    lo.2 = std::cmp::min(lo.2, pt.2 - 1);
    hi.0 = std::cmp::max(hi.0, pt.0 + 1);
    hi.1 = std::cmp::max(hi.1, pt.1 + 1);
    hi.2 = std::cmp::max(hi.2, pt.2 + 1);
  }
  let mut outside = Pts::new();
  let mut queue: Vec<Pt> = Vec::new();
  for x in lo.0..=hi.0 {
    for y in lo.1..=hi.1 {
      for z in lo.2..=hi.2 {
        let border =
          (x == lo.0 || x == hi.0) || (y == lo.1 || y == hi.1) || (z == lo.2 || z == hi.2);
        let p = (x, y, z);
        if border && !pts.contains(&p) {
          outside.insert(p);
          queue.push(p);
        }
      }
    }
  }
  let mut i = 0;
  while i < queue.len() {
    let p = queue[i];
    i += 1;
    for (d0, d1, d2) in DELTAS {
      let q = (p.0 + d0, p.1 + d1, p.2 + d2);
      if q.0 < lo.0 || q.0 > hi.0 || q.1 < lo.1 || q.1 > hi.1 || q.2 < lo.2 || q.2 > hi.2 {
        continue;
      }
      if !pts.contains(&q) && !outside.contains(&q) {
        outside.insert(q);
        queue.push(q);
      }
    }
  }
  let mut trapped = Pts::new();
  for x in lo.0..=hi.0 {
    for y in lo.1..=hi.1 {
      for z in lo.2..=hi.2 {
        let p = (x, y, z);
        if !pts.contains(&p) && !outside.contains(&p) {
          trapped.insert(p);
        }
      }
    }
  }
  trapped
}

fn surface(pts: &Pts, trapped: &Pts) -> i32 {
  let mut result = 0;
  for p in pts.iter() {
    for (d0, d1, d2) in DELTAS {
      let q = (p.0 + d0, p.1 + d1, p.2 + d2);
      if !trapped.contains(&q) && !pts.contains(&q) {
        result += 1;
      }
    }
  }
  result
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let pts: Pts = lines
    .iter()
    .map(|line| {
      let a: Vec<_> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
      (a[0], a[1], a[2])
    })
    .collect();
  let part1 = surface(&pts, &Pts::new());
  let part2 = surface(&pts, &get_trapped(&pts));
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day18, 64, 58;);
