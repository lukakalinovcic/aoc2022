fn solve(nums: &Vec<i64>, mult: i64, rounds: usize) -> i64 {
  let n = nums.len();
  let mut prev: Vec<_> = (0..n).map(|x| (x + n - 1) % n).collect();
  let mut next: Vec<_> = (0..n).map(|x| (x + n + 1) % n).collect();
  for _ in 0..rounds {
    for i in 0..n {
      let m = n as i64 - 1;
      let k = ((mult * nums[i]) % m + m) % m;
      for _ in 0..k {
        // [..., a, i, j, b, ...] ===> [..., a, j, i, b, ...]
        let j = next[i];
        let b = next[j];
        let a = prev[i];
        next[a] = j;
        next[j] = i;
        next[i] = b;
        prev[j] = a;
        prev[i] = j;
        prev[b] = i;
      }
    }
  }
  let mut result = 0;
  let mut i = nums.iter().position(|x| *x == 0).unwrap();
  for _ in 0..3 {
    for _ in 0..1000 {
      i = next[i];
    }
    result += mult * nums[i];
  }
  result
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  let nums: Vec<_> = lines
    .iter()
    .map(|line| line.parse::<i64>().unwrap())
    .collect();
  let part1 = solve(&nums, 1, 1);
  let part2 = solve(&nums, 811589153, 10);
  (part1.to_string(), part2.to_string())
}

extern crate macros;
macros::tests!(day20, 3, 1623178306;);
