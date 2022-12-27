enum Op {
  Add,
  Sub,
  Mult,
  Div,
}

enum Rhs {
  Leaf(i64),
  Binary(String, String, Op),
}

struct Node {
  name: String,
  rhs: Rhs,
}

impl Node {
  fn new(line: &String) -> Node {
    let (name, rhs_str) = line.split_once(": ").unwrap();
    for (s, op) in [
      (" + ", Op::Add),
      (" - ", Op::Sub),
      (" * ", Op::Mult),
      (" / ", Op::Div),
    ] {
      if let Some((a, b)) = rhs_str.split_once(s) {
        return Node {
          name: name.to_string(),
          rhs: Rhs::Binary(a.to_string(), b.to_string(), op),
        };
      }
    }
    Node {
      name: name.to_string(),
      rhs: Rhs::Leaf(rhs_str.parse().unwrap()),
    }
  }
}

type NodeMap = std::collections::HashMap<String, Node>;

fn eval(nodes: &NodeMap, name: &String, humn_override: Option<i64>) -> i64 {
  if name == "humn" {
    if let Some(v) = humn_override {
      return v;
    }
  }
  let node = nodes.get(name).unwrap();
  match &node.rhs {
    Rhs::Leaf(v) => *v,
    Rhs::Binary(a, b, op) => {
      let a = eval(nodes, a, humn_override);
      let b = eval(nodes, b, humn_override);
      match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mult => a * b,
        Op::Div => a / b,
      }
    }
  }
}

fn kids(nodes: &NodeMap, name: &String) -> Option<(String, String)> {
  let node = nodes.get(name).unwrap();
  match &node.rhs {
    Rhs::Leaf(_) => None,
    Rhs::Binary(a, b, _) => Some((a.clone(), b.clone())),
  }
}

fn loss(nodes: &NodeMap, lt: &String, rt: &String, humn_value: i64) -> i64 {
  let l = eval(&nodes, &lt, Some(humn_value));
  let r = eval(&nodes, &rt, Some(humn_value));
  (l - r).abs()
}

fn solve2(nodes: &NodeMap, lt: &String, rt: &String) -> Option<i64> {
  let (mut lo, mut hi) = (-1000000000000000, 1000000000000000);
  while hi - lo > 30 {
    let mid1 = (2 * lo + hi) / 3;
    let mid2 = (lo + 2 * hi) / 3;
    if loss(&nodes, &lt, &rt, mid1) < loss(&nodes, &lt, &rt, mid2) {
      hi = mid2;
    } else {
      lo = mid1;
    }
  }
  for v in lo..=hi {
    if loss(&nodes, &lt, &rt, v) == 0 {
      return Some(v);
    }
  }
  None
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let nodes = NodeMap::from_iter(
    lines
      .iter()
      .map(Node::new)
      .map(|node| (node.name.clone(), node)),
  );
  let root_str = String::from("root");
  let part1 = eval(&nodes, &root_str, None);

  let (lt, rt) = kids(&nodes, &root_str).unwrap();
  let part2 = solve2(&nodes, &lt, &rt).unwrap();

  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day21, 152, 301;);
