#[derive(Debug)]
enum Token {
  ListOpen,
  ListClose,
  Number(u32),
}

fn tokenize(s: &str) -> Vec<Token> {
  let flush = |out: &mut Vec<Token>, curr_value: &mut Option<u32>| {
    match *curr_value {
      Some(value) => out.push(Token::Number(value)),
      None => (),
    };
    *curr_value = None;
  };
  let mut curr_value: Option<u32> = None;
  let mut out: Vec<Token> = Vec::new();
  for c in s.chars() {
    match c {
      '[' => {
        flush(&mut out, &mut curr_value);
        out.push(Token::ListOpen);
      }
      ']' => {
        flush(&mut out, &mut curr_value);
        out.push(Token::ListClose);
      }
      ',' => flush(&mut out, &mut curr_value),
      c => match curr_value {
        Some(val) => curr_value = Some(val * 10 + c.to_digit(10).unwrap()),
        None => curr_value = Some(c.to_digit(10).unwrap()),
      },
    }
  }
  out
}

enum Node {
  Number(u32),
  List(Vec<Box<Node>>),
}

impl Node {
  fn new(tokens: Vec<Token>) -> Box<Node> {
    let mut stack: Vec<Vec<Box<Node>>> = vec![];

    for tok in tokens {
      match tok {
        Token::Number(v) => {
          let back = stack.len() - 1;
          stack[back].push(Box::new(Node::Number(v)))
        }
        Token::ListOpen => stack.push(Vec::new()),
        Token::ListClose => {
          let children = stack.pop().unwrap();
          let node = Box::new(Node::List(children));
          if stack.is_empty() {
            return node;
          }
          let back = stack.len() - 1;
          stack[back].push(node);
        }
      }
    }
    panic!("unreachable")
  }
}

impl std::fmt::Display for Node {
  fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Node::Number(v) => write!(f, "{}", v)?,
      Node::List(children) => {
        write!(f, "[")?;
        for (i, ch) in children.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          ch.fmt(f)?;
        }
        write!(f, "]")?
      }
    }
    Ok(())
  }
}

impl std::cmp::Ord for Node {
  fn cmp(self: &Self, other: &Self) -> std::cmp::Ordering {
    let (a, b) = (self, other);
    match (a, b) {
      (Node::Number(va), Node::Number(vb)) => va.cmp(&vb),
      (Node::List(cha), Node::List(chb)) => {
        let n = std::cmp::max(cha.len(), chb.len());
        for i in 0..n {
          if i >= cha.len() {
            return std::cmp::Ordering::Less;
          }
          if i >= chb.len() {
            return std::cmp::Ordering::Greater;
          }
          match cha[i].cmp(&chb[i]) {
            std::cmp::Ordering::Equal => (),
            cmp => return cmp,
          }
        }
        std::cmp::Ordering::Equal
      }
      (Node::Number(va), _) => Node::List(vec![Box::new(Node::Number(*va))]).cmp(b),
      (_, Node::Number(vb)) => a.cmp(&Node::List(vec![Box::new(Node::Number(*vb))])),
    }
  }
}

impl std::cmp::PartialOrd for Node {
  fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl std::cmp::Eq for Node {}

impl std::cmp::PartialEq for Node {
  fn eq(self: &Self, other: &Self) -> bool {
    self.cmp(other) == std::cmp::Ordering::Equal
  }
}

fn parse_pair(lines: &mut std::slice::Iter<String>) -> Option<(Vec<Token>, Vec<Token>)> {
  let t = lines.next()?;
  let l1 = if t.is_empty() {
    lines.next().unwrap()
  } else {
    t
  };
  let l2 = lines.next().unwrap();
  Some((tokenize(l1), tokenize(l2)))
}

fn solve1(lines: &Vec<String>) -> i32 {
  let mut iter = lines.iter();
  let mut result = 0;
  for i in 0.. {
    match parse_pair(&mut iter) {
      Some((t1, t2)) => {
        if Node::new(t1).cmp(&Node::new(t2)) != std::cmp::Ordering::Greater {
          result += i + 1;
        }
      }
      None => break,
    }
  }
  result
}

fn solve2(lines: &Vec<String>) -> i32 {
  let mut nodes: Vec<Box<Node>> = vec![Node::new(tokenize("[[2]]")), Node::new(tokenize("[[6]]"))];
  for line in lines {
    if !line.is_empty() {
      nodes.push(Node::new(tokenize(line)));
    }
  }
  nodes.sort();
  let mut i2 = -1;
  let mut i6 = -1;
  for (i, node) in nodes.iter().enumerate() {
    let node_str = format!("{}", node);
    if node_str == "[[2]]" {
      i2 = (i + 1) as i32;
    }
    if node_str == "[[6]]" {
      i6 = (i + 1) as i32;
    }
  }
  i2 * i6
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  (solve1(&lines).to_string(), solve2(&lines).to_string())
}

extern crate macros;
macros::tests!(day13, 13, 140;);
