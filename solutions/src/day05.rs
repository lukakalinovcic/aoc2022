use arr_macro::arr;

struct Ship {
  stacks: [Vec<char>; 10],
}

impl Ship {
  fn new(lines: &mut dyn Iterator<Item = &String>) -> Ship {
    let mut stacks = arr![Vec::new(); 10];
    for line in lines {
      if line.is_empty() {
        break;
      }
      for (i, ch) in line.chars().enumerate() {
        if ch.is_uppercase() {
          stacks[1 + i / 4].insert(0, ch);
        }
      }
    }
    Ship { stacks }
  }

  fn doit(&mut self, cnt: usize, from: usize, to: usize) {
    for _ in 0..cnt {
      let ch = self.stacks[from].pop().unwrap();
      self.stacks[to].push(ch);
    }
  }

  fn top(&self) -> String {
    let mut out = String::new();
    for stack in &self.stacks {
      if !stack.is_empty() {
        let ch = stack[stack.len() - 1];
        out.push(ch);
      }
    }
    out
  }
}

fn solve(lines: &Vec<String>, part1: bool) -> String {
  let mut lines = lines.iter();
  let mut ship = Ship::new(&mut lines);
  for line in lines {
    let parts: Vec<&str> = line.split(" ").collect();
    let k = parts[1].parse::<usize>().unwrap();
    let a = parts[3].parse::<usize>().unwrap();
    let b = parts[5].parse::<usize>().unwrap();
    if part1 {
      ship.doit(k, a, b);
    } else {
      ship.doit(k, a, 0);
      ship.doit(k, 0, b);
    }
  }
  ship.top()
}

pub fn doit(lines: &Vec<String>) -> (Option<String>, Option<String>) {
  (
    Some(solve(&lines, true).to_string()),
    Some(solve(&lines, false).to_string()),
  )
}

extern crate macros;
macros::make_test!(day05, "CMZ", "MCD");
