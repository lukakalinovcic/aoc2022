struct HeightMap {
  data: Vec<Vec<u8>>,
  rows: usize,
  cols: usize,
  start: (i32, i32),
  end: (i32, i32),
}

impl HeightMap {
  fn new(mut data: Vec<Vec<u8>>) -> Self {
    let (rows, cols) = (data.len(), data[0].len());
    let mut start = (-1, -1);
    let mut end = (-1, -1);
    for r in 0..rows {
      for c in 0..cols {
        if data[r][c] == b'S' {
          data[r][c] = b'a';
          start = (r as i32, c as i32);
        }
        if data[r][c] == b'E' {
          data[r][c] = b'z';
          end = (r as i32, c as i32);
        }
      }
    }
    Self {
      data,
      rows,
      cols,
      start,
      end,
    }
  }

  fn get(self: &Self, r: i32, c: i32) -> i32 {
    let (r, c) = (r as usize, c as usize);
    if r >= self.rows || c >= self.cols {
      return 0;
    }
    self.data[r][c] as i32
  }
}

fn bfs(map: &HeightMap) -> Vec<Vec<i32>> {
  let mut dist = vec![vec![i32::MAX; map.cols]; map.rows];
  let mut queue: Vec<(i32, i32)> = vec![map.end];
  dist[map.end.0 as usize][map.end.1 as usize] = 0;
  let mut i: usize = 0;
  while i < queue.len() {
    let (r, c) = queue[i];
    i += 1;
    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let (rr, cc) = (r + dr, c + dc);
      if (map.get(rr, cc) - map.get(r, c)) < -1 {
        continue;
      }
      if dist[rr as usize][cc as usize] != i32::MAX {
        continue;
      }
      dist[rr as usize][cc as usize] = dist[r as usize][c as usize] + 1;
      queue.push((rr, cc));
    }
  }
  dist
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let map = HeightMap::new(
    lines
      .into_iter()
      .map(|line| line.bytes().collect())
      .collect(),
  );
  let dist = bfs(&map);
  let mut min_dist = i32::MAX;
  for r in 0..map.rows {
    for c in 0..map.cols {
      if map.data[r][c] == b'a' {
        min_dist = std::cmp::min(min_dist, dist[r][c]);
      }
    }
  }
  (
    dist[map.start.0 as usize][map.start.1 as usize].to_string(),
    min_dist.to_string(),
  )
}

extern crate macros;
macros::tests!(day12, 31, 29;);
