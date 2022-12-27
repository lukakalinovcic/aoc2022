struct Blueprint {
  index: i32,
  ore_cost_ore: i32,
  clay_cost_ore: i32,
  obs_cost_ore: i32,
  obs_cost_clay: i32,
  geo_cost_ore: i32,
  geo_cost_obs: i32,
  max_cost_ore: i32,
}

impl Blueprint {
  fn from(line: &String) -> Blueprint {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let nums: Vec<_> = re
      .captures_iter(line)
      .map(|m| {
        m.iter()
          .next()
          .unwrap()
          .unwrap()
          .as_str()
          .parse::<i32>()
          .unwrap()
      })
      .collect();
    Blueprint {
      index: nums[0],
      ore_cost_ore: nums[1],
      clay_cost_ore: nums[2],
      obs_cost_ore: nums[3],
      obs_cost_clay: nums[4],
      geo_cost_ore: nums[5],
      geo_cost_obs: nums[6],
      max_cost_ore: *[nums[1], nums[2], nums[3], nums[5]].iter().max().unwrap(),
    }
  }
}

fn solve(
  b: &Blueprint,
  time: i32,
  ore_robots: i32,
  clay_robots: i32,
  obs_robots: i32,
  geo_robots: i32,
  ore: i32,
  clay: i32,
  obs: i32,
) -> i32 {
  if time < 0 {
    return i32::MIN;
  }
  let mut result = geo_robots * time;
  // Build ore robot next.
  if ore_robots < b.max_cost_ore && clay_robots + obs_robots < 2 {
    let w = 1 + std::cmp::max(0, (b.ore_cost_ore - ore + ore_robots - 1) / ore_robots);
    let t = geo_robots * w
      + solve(
        b,
        time - w,
        ore_robots + 1,
        clay_robots,
        obs_robots,
        geo_robots,
        ore + ore_robots * w - b.ore_cost_ore,
        clay + clay_robots * w,
        obs + obs_robots * w,
      );
    result = std::cmp::max(result, t);
  }
  // Build clay robot next.
  if clay_robots < b.obs_cost_clay && geo_robots < 2 {
    let w = 1 + std::cmp::max(0, (b.clay_cost_ore - ore + ore_robots - 1) / ore_robots);
    let t = geo_robots * w
      + solve(
        b,
        time - w,
        ore_robots,
        clay_robots + 1,
        obs_robots,
        geo_robots,
        ore + ore_robots * w - b.clay_cost_ore,
        clay + clay_robots * w,
        obs + obs_robots * w,
      );
    result = std::cmp::max(result, t);
  }
  // Build obs robot next.
  if clay_robots > 0 && obs_robots < b.geo_cost_obs {
    let w = 1
      + std::cmp::max(
        0,
        std::cmp::max(
          (b.obs_cost_ore - ore + ore_robots - 1) / ore_robots,
          (b.obs_cost_clay - clay + clay_robots - 1) / clay_robots,
        ),
      );
    let t = geo_robots * w
      + solve(
        b,
        time - w,
        ore_robots,
        clay_robots,
        obs_robots + 1,
        geo_robots,
        ore + ore_robots * w - b.obs_cost_ore,
        clay + clay_robots * w - b.obs_cost_clay,
        obs + obs_robots * w,
      );
    result = std::cmp::max(result, t);
  }
  // Build geo robot next.
  if obs_robots > 0 {
    let w = 1
      + std::cmp::max(
        0,
        std::cmp::max(
          (b.geo_cost_ore - ore + ore_robots - 1) / ore_robots,
          (b.geo_cost_obs - obs + obs_robots - 1) / obs_robots,
        ),
      );
    let t = geo_robots * w
      + solve(
        b,
        time - w,
        ore_robots,
        clay_robots,
        obs_robots,
        geo_robots + 1,
        ore + ore_robots * w - b.geo_cost_ore,
        clay + clay_robots * w,
        obs + obs_robots * w - b.geo_cost_obs,
      );
    result = std::cmp::max(result, t);
  }
  result
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let blueprints: Vec<_> = lines.iter().map(Blueprint::from).collect();
  let mut part1 = 0;
  for b in blueprints.iter() {
    part1 += b.index * solve(b, 24, 1, 0, 0, 0, 0, 0, 0);
  }
  let mut part2 = 1;
  let take = std::cmp::min(3, blueprints.len());
  for b in &blueprints[0..take] {
    let t = solve(b, 32, 1, 0, 0, 0, 0, 0, 0);
    part2 *= t;
  }
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day19, 33, 3472;);
