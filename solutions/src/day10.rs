pub fn doit(lines: &Vec<String>) -> (String, String) {
  let mut value = 1;
  let mut cycle = 1;
  let mut part1_result = 0;
  let mut crt = vec![vec!['.'; 40]; 6];
  let mut advance = |add| {
    let row = (cycle - 1) / 40;
    let col = (cycle - 1) % 40;
    if let -1..=1 = col - value {
      crt[row as usize][col as usize] = '#';
    }
    if cycle % 40 == 20 {
      part1_result += cycle * value;
    }
    cycle += 1;
    value += add;
  };
  for line in lines {
    let parts: Vec<&str> = line.split(" ").collect();
    advance(0);
    if parts[0] == "addx" {
      advance(parts[1].parse::<i32>().unwrap());
    }
  }
  (
    part1_result.to_string(),
    crt
      .iter()
      .map(|row| row.iter().collect::<String>())
      .fold(String::new(), |a, b| format!("{a}\n{b}")),
  )
}

extern crate macros;
macros::tests!(
  day10, 13140, r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
);
