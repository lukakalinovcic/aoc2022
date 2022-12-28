const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;
const DELTAS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Context {
  map: Vec<Vec<u8>>,
  face_size: usize,
  faces: Vec<Face>,
}

struct Face {
  r0: usize,
  c0: usize,
  edge: [Option<Edge>; 4],
}

impl Face {
  fn new(r0: usize, c0: usize) -> Face {
    Face {
      r0,
      c0,
      edge: [None, None, None, None],
    }
  }
}

#[derive(Clone, Copy)]
struct Edge {
  face: usize,
  side: usize,
  flip: bool,
}

struct Position {
  face: usize,
  r: i32,
  c: i32,
  dir: usize,
}

impl Edge {
  fn new(face: usize, side: usize, flip: bool) -> Edge {
    Edge { face, side, flip }
  }

  fn cross(&self, ctx: &Context, i: i32) -> Position {
    let sz = ctx.face_size as i32;
    let i = if self.flip { sz - i - 1 } else { i };
    match self.side {
      RIGHT => Position::new(self.face, i, sz - 1, LEFT),
      DOWN => Position::new(self.face, sz - 1, sz - i - 1, UP),
      LEFT => Position::new(self.face, sz - i - 1, 0, RIGHT),
      UP => Position::new(self.face, 0, i, DOWN),
      _ => panic!("unknown side"),
    }
  }
}

impl Position {
  fn new(face: usize, r: i32, c: i32, dir: usize) -> Position {
    Position { face, r, c, dir }
  }

  fn fw(&self, ctx: &Context) -> Position {
    let sz = ctx.face_size as i32;
    let face = &ctx.faces[self.face];
    match (self.r + DELTAS[self.dir].0, self.c + DELTAS[self.dir].1) {
      (r, c) if c == sz => face.edge[RIGHT].as_ref().unwrap().cross(ctx, r),
      (r, c) if r == sz => face.edge[DOWN].as_ref().unwrap().cross(ctx, sz - c - 1),
      (r, c) if c == -1 => face.edge[LEFT].as_ref().unwrap().cross(ctx, sz - r - 1),
      (r, c) if r == -1 => face.edge[UP].as_ref().unwrap().cross(ctx, c),
      (r, c) => Position::new(self.face, r, c, self.dir),
    }
  }

  fn is_open(&self, ctx: &Context) -> bool {
    let face = &ctx.faces[self.face];
    ctx.map[face.r0 + self.r as usize][face.c0 + self.c as usize] == b'.'
  }

  fn password(&self, ctx: &Context) -> usize {
    let face = &ctx.faces[self.face];
    let r = face.r0 + self.r as usize + 1;
    let c = face.c0 + self.c as usize + 1;
    return 1000 * r + 4 * c + self.dir;
  }
}

fn parse_input(map: &[String], glue_faces: fn(Vec<Face>) -> Vec<Face>) -> Context {
  let map: Vec<Vec<u8>> = map
    .iter()
    .map(|line| line.as_bytes().iter().cloned().collect::<Vec<u8>>())
    .collect();
  let area: i32 = map
    .iter()
    .map(|row| {
      (*row)
        .iter()
        .map(|c| if *c == b' ' { 0 } else { 1 })
        .sum::<i32>()
    })
    .sum();
  let face_size = ((area / 6) as f32).sqrt() as usize;

  let mut faces: Vec<Face> = Vec::new();
  for r0 in (0..).step_by(face_size) {
    if r0 >= map.len() {
      break;
    }
    for c0 in (0..).step_by(face_size) {
      if c0 >= map[r0].len() {
        break;
      }
      if map[r0][c0] == b' ' {
        continue;
      }
      faces.push(Face::new(r0, c0));
    }
  }
  for i in 0..faces.len() {
    for j in i + 1..faces.len() {
      if faces[i].r0 == faces[j].r0 && faces[i].c0 + face_size == faces[j].c0 {
        faces[i].edge[RIGHT] = Some(Edge::new(j, LEFT, true));
        faces[j].edge[LEFT] = Some(Edge::new(i, RIGHT, true));
      }
      if faces[i].r0 + face_size == faces[j].r0 && faces[i].c0 == faces[j].c0 {
        faces[i].edge[DOWN] = Some(Edge::new(j, UP, true));
        faces[j].edge[UP] = Some(Edge::new(i, DOWN, true));
      }
    }
  }
  let faces = glue_faces(faces);
  Context {
    map,
    face_size,
    faces,
  }
}

fn glue_faces_part1(mut faces: Vec<Face>) -> Vec<Face> {
  for i in 0..6 {
    for side in 0..4 {
      if let Some(_) = faces[i].edge[side] {
        continue;
      }
      let mut j = i;
      while let Some(Edge { face: k, .. }) = faces[j].edge[side ^ 2] {
        j = k;
      }
      faces[i].edge[side] = Some(Edge::new(j, side ^ 2, true));
      faces[j].edge[side ^ 2] = Some(Edge::new(i, side, true));
    }
  }
  faces
}
fn glue_faces_part2(mut faces: Vec<Face>) -> Vec<Face> {
  for _ in 0..10 {
    for i in 0..6 {
      for a in 0..4 {
        let b = (a + 1) % 4;
        let (a, b) = match (faces[i].edge[a], faces[i].edge[b]) {
          (Some(e1), Some(e2)) => (e1, e2),
          _ => continue,
        };
        let aside = (a.side + if a.flip { 3 } else { 1 }) % 4;
        let bside = (b.side + if b.flip { 1 } else { 3 }) % 4;
        if let (None, None) = (faces[a.face].edge[aside], faces[b.face].edge[bside]) {
          faces[a.face].edge[aside] = Some(Edge::new(b.face, bside, !(a.flip ^ b.flip)));
          faces[b.face].edge[bside] = Some(Edge::new(a.face, aside, !(a.flip ^ b.flip)));
        }
      }
    }
  }
  faces
}

enum Command {
  Fw(i32),
  Turn(usize),
}

fn parse_commands(s: &String) -> Vec<Command> {
  let mut commands = Vec::new();
  for c in s.bytes() {
    match c {
      b'L' => commands.push(Command::Turn(3)),
      b'R' => commands.push(Command::Turn(1)),
      c => {
        let last_fw = if let Some(Command::Fw(v)) = commands.last() {
          Some(*v)
        } else {
          None
        };
        match last_fw {
          Some(v) => {
            commands.pop();
            commands.push(Command::Fw(v * 10 + (c - b'0') as i32));
          }
          _ => commands.push(Command::Fw((c - b'0') as i32)),
        }
      }
    }
  }
  commands
}

fn walk(ctx: &Context, commands: &Vec<Command>) -> usize {
  let mut p = Position::new(0, 0, 0, 0);
  for command in commands {
    match *command {
      Command::Turn(d) => p.dir = (p.dir + d) % 4,
      Command::Fw(steps) => {
        for _ in 0..steps {
          let q = p.fw(ctx);
          if q.is_open(ctx) {
            p = q;
          }
        }
      }
    }
  }
  p.password(ctx)
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let map = &lines[..lines.len() - 2];
  let commands = parse_commands(&lines[lines.len() - 1]);
  let part1 = walk(&parse_input(&map, glue_faces_part1), &commands);
  let part2 = walk(&parse_input(&map, glue_faces_part2), &commands);
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day22, 6032, 5031;);
