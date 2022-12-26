extern crate proc_macro;
use quote::{quote, ToTokens, TokenStreamExt};

#[proc_macro]
pub fn register_solutions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = proc_macro2::TokenStream::from(input);
  let n: usize = input.to_string().parse().unwrap();
  let mut mod_output = proc_macro2::TokenStream::new();
  let mut fn_output = proc_macro2::TokenStream::new();
  for i in 1..=n {
    let name: proc_macro2::TokenStream = format!("day{:02}", i).parse().unwrap();
    mod_output.extend(quote! {
      pub mod #name;
    });
    fn_output.extend(quote! {
      crate::#name::doit,
    })
  }
  proc_macro::TokenStream::from(quote! {
    pub const SOLUTIONS: usize = #n;
    #mod_output
    static REGISTRY: &'static [fn(&Vec<String>) -> (String, String)] = &[
      #fn_output
    ];
  })
}

struct Want(Option<String>, Option<String>);
impl ToTokens for Want {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let to_str = |w: &Option<String>| match w {
      Some(v) => {
        if v.starts_with("\"") {
          format!("{v}.to_string()")
        } else {
          format!("\"{v}\".to_string()")
        }
      }
      None => "None".to_string(),
    };
    let line = format!("({}, {})", to_str(&self.0), to_str(&self.1));
    for t in line.parse::<proc_macro2::TokenStream>().unwrap() {
      tokens.append(t);
    }
  }
}

#[proc_macro]
pub fn tests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = proc_macro2::TokenStream::from(input);
  let trees: Vec<proc_macro2::TokenTree> = input.into_iter().collect();
  let mut tests_output = proc_macro2::TokenStream::new();
  let mut basename: Option<proc_macro2::Ident> = None;
  let mut want = Want(None, None);
  for tree in trees {
    match tree {
      proc_macro2::TokenTree::Ident(v) => basename = Some(v),
      proc_macro2::TokenTree::Literal(v) => {
        if let None = want.0 {
          want.0 = Some(v.to_string());
        } else {
          want.1 = Some(v.to_string());
        }
      }
      proc_macro2::TokenTree::Punct(v) => {
        if v.to_string() == ";" {
          let filename = format!("../testdata/{}.in", basename.as_ref().unwrap().to_string());
          tests_output.extend(quote! {
            #[test]
            fn #basename() {
              let got = super::doit(&read_lines(#filename));
              assert_eq!(got, #want);
            }
          });
          want = Want(None, None);
          basename = None;
        }
      }
      _ => (),
    }
  }

  proc_macro::TokenStream::from(quote! {
    #[cfg(test)]
    mod tests {
      fn read_lines(filename: &str) -> Vec<String> {
        use std::io::BufRead;
        let file = std::fs::File::open(filename).unwrap();
        std::io::BufReader::new(file)
          .lines()
          .map(|line| line.unwrap())
          .collect()
      }

      #tests_output
    }
  })
}
