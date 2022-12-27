type Pt = (i32, i32);
type SnB = (Pt, Pt);

fn scan_line(snbs: &Vec<SnB>, ty: i32, sz: i32) -> (i32, Option<i32>) {
  const START: u8 = 1;
  const END: u8 = 2;
  let mut events: Vec<(i32, u8)> = Vec::new();
  let mut beacons = std::collections::HashSet::<i32>::new();
  for ((sx, sy), (bx, by)) in snbs {
    if *by == ty {
      beacons.insert(*bx);
    }
    let d = (sx - bx).abs() + (sy - by).abs();
    let w = d - (ty - sy).abs();
    if w >= 0 {
      events.push((sx - w, START));
      events.push((sx + w + 1, END));
    }
  }
  events.sort();

  let mut active = 0;
  let mut prev_x = i32::MIN;
  let mut covered = 0;
  let mut uncovered: Option<i32> = None;
  for (x, t) in events {
    if active > 0 {
      covered += x - prev_x;
    }
    active = if t == START { active + 1 } else { active - 1 };
    if active == 0 && x >= 0 && x <= sz {
      uncovered = Some(x);
    }
    prev_x = x;
  }
  (covered - beacons.len() as i32, uncovered)
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let re = regex::Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
  let mid: i32 = lines[0].parse().unwrap();
  let snbs: Vec<SnB> = lines
    .iter()
    .skip(1)
    .map(|line| {
      let pts: Vec<_> = re
        .captures_iter(line)
        .map(|m| {
          (
            m.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            m.get(2).unwrap().as_str().parse::<i32>().unwrap(),
          )
        })
        .collect();
      (pts[0], pts[1])
    })
    .collect();

  let part1 = scan_line(&snbs, mid, 2 * mid).0;
  let mut part2: Option<Pt> = None;
  for y in 0..=(2 * mid) {
    if let Some(x) = scan_line(&snbs, y, 2 * mid).1 {
      part2 = Some((x, y));
    }
  }
  let part2 = part2.unwrap();
  let part2 = part2.0 as i64 * 4000000 + part2.1 as i64;
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day15, 26, 56000011;);
