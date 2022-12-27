type LineData = (String, i32, Vec<String>);
type Ids = std::collections::HashMap<String, usize>;

struct Graph {
  data: Vec<(i32, Vec<usize>)>,
}

impl Graph {
  fn new(lines: &Vec<LineData>, ids: &Ids) -> Graph {
    let mut data = vec![(0, vec![]); ids.len()];
    for (name, flow, adj) in lines {
      let id = *ids.get(name).unwrap();
      data[id].0 = *flow;
      data[id].1.push(id);
      for name in adj {
        data[id].1.push(*ids.get(name).unwrap());
      }
    }
    Graph { data }
  }
}

type Memo = std::collections::HashMap<(u8, u8, u8, u64), i32>;

fn solve(
  graph: &Graph,
  t: usize,
  p1: usize,
  p2: usize,
  mask: usize,
  add: i32,
  use_pruning: bool,
  memo: &mut Memo,
) -> i32 {
  if t == 0 {
    return 0;
  }
  // Dirty hack!
  if use_pruning && t >= 10 && t <= 20 && add < (200 - t as i32 * 8) {
    return 0;
  }
  let memo_key = (t as u8, p1 as u8, p2 as u8, mask as u64);
  if let Some(result) = memo.get(&memo_key) {
    return *result;
  }

  let mut result = 0;
  for np1 in &graph.data[p1].1 {
    if *np1 == p1 && (graph.data[p1].0 == 0 || (mask >> p1) & 1 > 0) {
      continue;
    }
    let nm = if *np1 == p1 { mask | (1 << p1) } else { mask };
    let na = add + if *np1 == p1 { graph.data[p1].0 } else { 0 };
    for np2 in &graph.data[p2].1 {
      if *np2 == p2 && (graph.data[p2].0 == 0 || (nm >> p2) & 1 > 0) {
        continue;
      }
      let nm = if *np2 == p2 { nm | (1 << p2) } else { nm };
      let na = na + if *np2 == p2 { graph.data[p2].0 } else { 0 };
      result = std::cmp::max(
        result,
        add + solve(graph, t - 1, *np1, *np2, nm, na, use_pruning, memo),
      );
    }
  }
  memo.insert(memo_key, result);
  result
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let re = regex::Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
    .unwrap();
  let mut lines: Vec<LineData> = lines
    .iter()
    .map(|line| {
      let m = re.captures(line).unwrap();
      (
        m.get(1).unwrap().as_str().to_string(),
        m.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        m.get(3)
          .unwrap()
          .as_str()
          .split(", ")
          .map(ToString::to_string)
          .collect(),
      )
    })
    .collect();
  lines.push(("NOOP1".to_string(), 0, vec!["NOOP2".to_string()]));
  lines.push(("NOOP2".to_string(), 0, vec!["NOOP1".to_string()]));

  let mut ids = Ids::new();
  for (name, flow, _) in lines.iter() {
    let id = ids.len();
    ids.insert(name.clone(), id);
  }
  let start = *ids.get("AA").unwrap();
  let noop = *ids.get("NOOP1").unwrap();

  let graph = Graph::new(&lines, &ids);
  let mut memo = Memo::new();
  let part1 = solve(&graph, 30, start, noop, 0, 0, false, &mut memo);

  let use_pruning = lines.len() > 30;
  let part2 = solve(&graph, 26, start, start, 0, 0, use_pruning, &mut memo);

  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day16, 1651, 1707;);
