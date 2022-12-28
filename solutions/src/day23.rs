type Pt = (i32, i32);
type Pts = std::collections::HashSet<Pt>;

fn parse_input(lines: &Vec<String>) -> Pts {
  Pts::from_iter(
    lines
      .iter()
      .enumerate()
      .map(|(y, row)| {
        row
          .as_bytes()
          .iter()
          .enumerate()
          .filter(|(_, v)| **v == b'#')
          .map(move |(x, _)| (x as i32, y as i32))
      })
      .flatten(),
  )
}

fn spread(pts: &Pts, round: usize) -> Pts {
  const DELTAS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
  let mut cnt = std::collections::HashMap::<Pt, usize>::new();
  let proposals = Vec::from_iter(pts.iter().cloned().map(|(x, y)| {
    if (-1..=1).any(|dx| (-1..=1).any(|dy| (dx != 0 || dy != 0) && pts.contains(&(x + dx, y + dy))))
    {
      for i in 0..4 {
        let (dx, dy) = DELTAS[(round + i - 1) % 4];
        if (-1..=1).all(|k| !pts.contains(&(x + dx + k * dy, y + dy + k * dx))) {
          let p = (x + dx, y + dy);
          *cnt.entry(p).or_default() += 1;
          return ((x, y), Some(p));
        }
      }
    }
    ((x, y), None)
  }));

  Pts::from_iter(proposals.iter().map(|(p, proposal)| match proposal {
    Some(q) if cnt[q] == 1 => *q,
    _ => *p,
  }))
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let mut pts = parse_input(lines);
  let (mut part1, mut part2) = (0, 0);
  for round in 1.. {
    let new_pts = spread(&pts, round);
    let done = new_pts.eq(&pts);
    pts = new_pts;
    if round == 10 {
      let mut mins = (i32::MAX, i32::MAX);
      let mut maxs = (i32::MIN, i32::MIN);
      for pt in &pts {
        mins.0 = std::cmp::min(mins.0, pt.0);
        mins.1 = std::cmp::min(mins.1, pt.1);
        maxs.0 = std::cmp::max(maxs.0, pt.0);
        maxs.1 = std::cmp::max(maxs.1, pt.1);
      }
      part1 = (maxs.0 - mins.0 + 1) * (maxs.1 - mins.1 + 1) - pts.len() as i32;
    }
    if done {
      part2 = round;
      break;
    }
  }
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day23, 110, 20;);
