fn part1(lines: &Vec<String>) -> i32 {
  let mut sum = 0;
  for line in lines {
    let p: Vec<&str> = line.split(" ").collect();
    let a = (p[0].chars().next().unwrap() as i32) - ('A' as i32);
    let b = (p[1].chars().next().unwrap() as i32) - ('X' as i32);
    sum += b + 1 + 3 * ((b - a + 4) % 3);
  }
  sum
}

fn part2(lines: &Vec<String>) -> i32 {
  let mut sum = 0;
  for line in lines {
    let p: Vec<&str> = line.split(" ").collect();
    let a = (p[0].chars().next().unwrap() as i32) - ('A' as i32);
    let x = (p[1].chars().next().unwrap() as i32) - ('X' as i32);
    let b = (a + x + 2) % 3;
    sum += b + 1 + 3 * ((b - a + 4) % 3);
  }
  sum
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  (part1(lines).to_string(), part2(lines).to_string())
}

extern crate macros;
macros::tests!(day02, 15, 12;);
