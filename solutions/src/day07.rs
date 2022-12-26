use std::collections::HashMap;

fn subdir(path: &str, name: &str) -> String {
  format!("{}/{}", path, name)
}

fn parent(path: &str) -> String {
  path.rsplit_once("/").unwrap().0.to_string()
}

struct Dir {
  files: HashMap<String, i64>,
  subdirs: Vec<String>,
}

impl Dir {
  fn new() -> Self {
    Dir {
      files: HashMap::<String, i64>::new(),
      subdirs: Vec::<String>::new(),
    }
  }
}

struct Dirs {
  path_to_dir: HashMap<String, Dir>,
}

impl Dirs {
  fn new(lines: &Vec<String>) -> Self {
    let mut path_to_dir = HashMap::<String, Dir>::new();
    let mut curr = "".to_string();
    path_to_dir.insert(curr.clone(), Dir::new());
    for line in lines {
      let dir = path_to_dir.get_mut(&curr).unwrap();
      let parts: Vec<&str> = line.split(" ").collect();
      if parts.len() == 3 {
        if parts[2] == ".." {
          // $ cd ..
          curr = parent(&curr);
        } else {
          // $ cd <subdir>
          curr = subdir(&curr, parts[2]);
        }
      } else {
        if parts[0] == "$" { // $ ls
           // Do nothing
        } else if parts[0] == "dir" {
          // dir <name>
          dir.subdirs.push(parts[1].to_string());
          path_to_dir.insert(subdir(&curr, parts[1]), Dir::new());
        } else {
          // <size> <name>
          dir
            .files
            .insert(parts[1].to_string(), parts[0].parse().unwrap());
        }
      }
    }
    Dirs { path_to_dir }
  }

  fn sizes(self: &Self, path: String, out: &mut Vec<i64>) -> i64 {
    let dir = self.path_to_dir.get(&path).unwrap();
    let mut dir_size = 0;
    for (_, file_size) in &dir.files {
      dir_size += file_size;
    }
    let dir = self.path_to_dir.get(&path).unwrap();
    for name in &dir.subdirs {
      dir_size += self.sizes(subdir(&path, &name), out);
    }
    out.push(dir_size);
    dir_size
  }
}

pub fn doit(lines: &Vec<String>) -> (Option<String>, Option<String>) {
  let dirs = Dirs::new(&lines);

  let mut dir_sizes = Vec::<i64>::new();
  dirs.sizes("".to_string(), &mut dir_sizes);

  let mut result1 = 0;
  for dir_size in &dir_sizes {
    if *dir_size <= 100000 {
      result1 += *dir_size;
    }
  }

  let free_space = 70000000 - dir_sizes[dir_sizes.len() - 1];
  let need_space = 30000000 - free_space;
  let mut result2 = 70000000;
  for dir_size in &dir_sizes {
    if *dir_size >= need_space && *dir_size < result2 {
      result2 = *dir_size
    }
  }
  (Some(result1.to_string()), Some(result2.to_string()))
}

extern crate macros;
macros::make_test!(day07, 95437, 24933642);
