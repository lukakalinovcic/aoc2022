type LineData = (String, i16, Vec<String>);
type Ids = std::collections::HashMap<String, usize>;

struct Graph {
  data: Vec<(i16, Vec<usize>)>,
}

impl Graph {
  fn new(lines: &Vec<LineData>, ids: &Ids) -> Graph {
    let mut data = vec![(0, vec![]); ids.len()];
    for (name, flow, adj) in lines {
      let id = *ids.get(name).unwrap();
      data[id].0 = *flow;
      for name in adj {
        data[id].1.push(*ids.get(name).unwrap());
      }
    }
    Graph { data }
  }
}

type Memo = multiarray::Array3D<i16>;

fn solve(graph: &Graph, t: usize, p: usize, mask: usize, memo: &mut Memo) -> i16 {
  if t == 0 {
    return 0;
  }
  {
    let cached = memo[[t, p, mask]];
    if cached >= 0 {
      return cached;
    }
  }

  let mut result = 0;
  if (mask >> p) & 1 > 0 {
    let add = (t - 1) as i16 * graph.data[p].0;
    result = add + solve(graph, t - 1, p, mask ^ (1 << p), memo);
  }
  for np in &graph.data[p].1 {
    result = std::cmp::max(result, solve(graph, t - 1, *np, mask, memo));
  }

  memo[[t, p, mask]] = result;
  result
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let re = regex::Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
    .unwrap();
  let lines: Vec<LineData> = lines
    .iter()
    .map(|line| {
      let m = re.captures(line).unwrap();
      (
        m.get(1).unwrap().as_str().to_string(),
        m.get(2).unwrap().as_str().parse::<i16>().unwrap(),
        m.get(3)
          .unwrap()
          .as_str()
          .split(", ")
          .map(ToString::to_string)
          .collect(),
      )
    })
    .collect();

  let mut ids = Ids::new();
  for pass in 0..2 {
    for i in 0..lines.len() {
      if pass == 0 && lines[i].1 > 0 || pass == 1 && lines[i].1 == 0 {
        let id = ids.len();
        ids.insert(lines[i].0.clone(), id);
      }
    }
  }
  let non_zero = lines.iter().filter(|d| d.1 > 0).count();
  let full_mask = (1 << non_zero) - 1;

  let start = *ids.get("AA").unwrap();
  let graph = Graph::new(&lines, &ids);

  let mut memo = Memo::new([31, ids.len(), (1 << non_zero)], -1);
  let part1 = solve(&graph, 30, start, full_mask, &mut memo);

  let mut part2 = 0;
  for mask1 in 0..(1 << (non_zero - 1)) {
    let mask2 = full_mask ^ mask1;
    part2 = std::cmp::max(
      part2,
      solve(&graph, 26, start, mask1, &mut memo) + solve(&graph, 26, start, mask2, &mut memo),
    )
  }

  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day16, 1651, 1707;);
