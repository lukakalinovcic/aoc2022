fn decode(s: &String) -> i64 {
  let mut result: i64 = 0;
  for c in s.bytes() {
    let add = match c {
      b'=' => -2,
      b'-' => -1,
      b'0' => 0,
      b'1' => 1,
      b'2' => 2,
      _ => panic!("unexpected"),
    };
    result = result * 5 + add;
  }
  result
}

fn encode(mut x: i64) -> String {
  let mut result = Vec::<u8>::new();
  while x > 0 {
    let (add, digit) = match x % 5 {
      0 => (0, b'0'),
      1 => (0, b'1'),
      2 => (0, b'2'),
      3 => (1, b'='),
      4 => (1, b'-'),
      _ => panic!("unexpected"),
    };
    result.push(digit);
    x = x / 5 + add;
  }
  result.reverse();
  String::from_utf8(result).unwrap()
}

pub fn doit(lines: &Vec<String>) -> (String, String) {
  (
    encode(lines.iter().map(decode).sum()),
    "Start the Blender".to_string(),
  )
}

extern crate macros;
macros::tests!(day25, "2=-1=0", "Start the Blender";);
