extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

#[proc_macro]
pub fn register_solutions(input: TokenStream) -> TokenStream {
  let n: usize = input.to_string().parse().unwrap();
  let mut code = String::new();
  for i in 1..=n {
    code.push_str(format!("pub mod day{:02};\n", i).as_str());
  }
  code.push_str("static REGISTRY: &'static [fn(&Vec<String>) -> (Option<String>, Option<String>)] = &[\n");
  for i in 1..=n {
    code.push_str(format!("  crate::day{:02}::doit,\n", i).as_str());
  }
  code.push_str("];\n");
  code.parse().unwrap()
}

struct Want(Option<String>, Option<String>);
impl ToTokens for Want {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let to_str = |w: &Option<String>| {
      match w {
        Some(v) => {
          if v.starts_with("\"") {
            format!("Some({v}.to_string())")
          } else {
            format!("Some(\"{v}\".to_string())")
          }
        },
        None => "None".to_string(),
      }
    };
    let line = format!("({}, {})", to_str(&self.0), to_str(&self.1));
    for t in line.parse::<proc_macro2::TokenStream>().unwrap() {
      tokens.append(t);
    }
  }
}

#[proc_macro]
pub fn make_test(input: TokenStream) -> TokenStream {
  let trees: Vec<TokenTree> = input.into_iter().collect();
  let mut filename = String::new();
  let mut want = Want(None, None);
  for tree in trees {
    match tree {
      TokenTree::Ident(v) => {
        filename = format!("../testdata/{}.in", v.to_string());
      },
      TokenTree::Literal(v) => {
        if let None = want.0 {
          want.0 = Some(v.to_string());
        } else {
          want.1 = Some(v.to_string());
        }
      },
      _ => (),
    }
  }
  
  let gen = quote! {
    #[cfg(test)]
    mod tests {
      fn read_lines(n: usize) -> Vec<String> {
        use std::fs::File;
        use std::io::{self, BufRead};
        let file = File::open(#filename).unwrap();
        io::BufReader::new(file)
          .lines()
          .map(|line| line.unwrap())
          .collect()
      }
     
      #[test]
      fn sample() {
        use super::*;
        let want = #want;
        let got = doit(&read_lines(1));
        assert_eq!(got, want);
      }
    }
  };
  gen.into()
}