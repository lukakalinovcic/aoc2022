use std::collections::HashMap;

enum Operation {
  Add(i32),
  Mult(i32),
  Square,
}

impl Operation {
  fn new(line: &str) -> Operation {
    match line.rsplit_once(" ").unwrap().1 {
      "old" => Operation::Square,
      tail => {
        let v: i32 = tail.parse().unwrap();
        match line.contains("+") {
          true => Operation::Add(v),
          false => Operation::Mult(v),
        }
      }
    }
  }

  fn apply(self: &Self, x: i32) -> i32 {
    match self {
      Operation::Add(v) => x + v,
      Operation::Mult(v) => x * v,
      Operation::Square => x * x,
    }
  }
}

struct ThrowTest {
  n: i32,
  true_monkey: usize,
  false_monkey: usize,
}

impl ThrowTest {
  fn new(line0: &str, line1: &str, line2: &str) -> ThrowTest {
    ThrowTest {
      n: line0.rsplit_once(" ").unwrap().1.parse().unwrap(),
      true_monkey: line1.rsplit_once(" ").unwrap().1.parse().unwrap(),
      false_monkey: line2.rsplit_once(" ").unwrap().1.parse().unwrap(),
    }
  }

  fn throw_to(self: &Self, v: i32) -> usize {
    match v % self.n {
      0 => self.true_monkey,
      _ => self.false_monkey,
    }
  }
}

trait Item {
  fn new(v: i32) -> Self;
  fn apply_op(self: &mut Self, op: &Operation);
  fn throw_to(self: &Self, test: &ThrowTest) -> usize;
}

#[derive(Clone)]
struct ItemPart1 {
  value: i32,
}

impl Item for ItemPart1 {
  fn new(v: i32) -> ItemPart1 {
    ItemPart1 { value: v }
  }

  fn apply_op(self: &mut Self, op: &Operation) {
    self.value = op.apply(self.value) / 3;
  }

  fn throw_to(self: &Self, test: &ThrowTest) -> usize {
    test.throw_to(self.value)
  }
}

#[derive(Clone)]
struct ItemPart2 {
  mod_values: HashMap<i32, i32>,
}

impl Item for ItemPart2 {
  fn new(v: i32) -> ItemPart2 {
    let mut mod_values: HashMap<i32, i32> = HashMap::new();
    for k in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29] {
      mod_values.insert(k, v % k);
    }
    ItemPart2 { mod_values }
  }

  fn apply_op(self: &mut Self, op: &Operation) {
    for (k, v) in &mut self.mod_values {
      *v = op.apply(*v) % *k;
    }
  }

  fn throw_to(self: &Self, test: &ThrowTest) -> usize {
    test.throw_to(*self.mod_values.get(&test.n).unwrap())
  }
}

struct Monkey<T: Item> {
  items: Vec<T>,
  op: Operation,
  test: ThrowTest,
}

impl<T: Item> Monkey<T> {
  fn parse(lines: &mut core::slice::Iter<String>) -> Option<Monkey<T>> {
    // Monkey 0:
    if lines.next()?.is_empty() {
      lines.next();
    }
    //   Starting items: 79, 98
    let s = lines.next().unwrap().rsplit_once(": ").unwrap().1;
    let items = s.split(", ").map(|s| T::new(s.parse().unwrap())).collect();
    //   Operation: new = old * 19
    let op = Operation::new(&lines.next().unwrap());
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    let test = ThrowTest::new(
      &lines.next().unwrap(),
      &lines.next().unwrap(),
      &lines.next().unwrap(),
    );

    Some(Monkey { items, op, test })
  }
}

fn parse<T: Item>(lines: &Vec<String>) -> Vec<Monkey<T>> {
  let mut lines = lines.iter();
  let mut monkeys = Vec::new();
  loop {
    match Monkey::<T>::parse(&mut lines) {
      Some(monkey) => monkeys.push(monkey),
      None => break,
    }
  }
  monkeys
}

fn solve<T: Item + Clone>(mut monkeys: Vec<Monkey<T>>, rounds: i32) -> i64 {
  let mut cnt = vec![0; monkeys.len()];
  for _ in 0..rounds {
    for i in 0..monkeys.len() {
      cnt[i] += monkeys[i].items.len() as i64;
      for mut item in monkeys[i].items.clone() {
        item.apply_op(&monkeys[i].op);
        let j = item.throw_to(&monkeys[i].test);
        monkeys[j].items.push(item);
      }
      monkeys[i].items.clear();
    }
  }
  cnt.sort();
  cnt.reverse();
  cnt[0] * cnt[1]
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  (
    solve(parse::<ItemPart1>(&lines), 20).to_string(),
    solve(parse::<ItemPart2>(&lines), 10000).to_string(),
  )
}

extern crate macros;
macros::tests!(day11, 10605, 2713310158;);
