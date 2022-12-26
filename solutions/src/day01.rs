pub fn doit(lines: &Vec<String>) -> (Option<String>, Option<String>) {
  let mut sums = Vec::new();
  let mut sum: i32 = 0;
  for s in lines {
    if s.len() == 0 {
      sums.push(sum);
      sum = 0;
    } else {
      let num = s.parse::<i32>().unwrap();
      sum += num;
    }
  }
  sums.push(sum);
  sums.sort();
  sums.reverse();
  (
    Some(sums[0].to_string()),
    Some((sums[0] + sums[1] + sums[2]).to_string()),
  )
}

extern crate macros;
macros::make_test!(day01, 24000, 45000);
