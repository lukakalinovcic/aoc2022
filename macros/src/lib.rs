extern crate proc_macro;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;

#[proc_macro]
pub fn register_solutions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let n: usize = TokenStream::from(input).to_string().parse().unwrap();
  let mut mod_output = TokenStream::new();
  let mut fn_output = TokenStream::new();
  for i in 1..=n {
    let name: TokenStream = format!("day{:02}", i).parse().unwrap();
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

#[proc_macro]
pub fn tests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let mut tests_output = TokenStream::new();
  let mut basename: Option<Ident> = None;
  let mut want0: Option<Literal> = None;
  let mut want1: Option<Literal> = None;
  for tree in TokenStream::from(input).into_iter() {
    match tree {
      proc_macro2::TokenTree::Ident(v) => basename = Some(v),
      proc_macro2::TokenTree::Literal(v) => {
        let num_to_str = |v: Literal| {
          let s = v.to_string();
          if s.contains("\"") {
            v
          } else {
            format!("\"{s}\"").parse().unwrap()
          }
        };
        if let None = want0 {
          want0 = Some(num_to_str(v));
        } else {
          want1 = Some(num_to_str(v));
        }
      }
      proc_macro2::TokenTree::Punct(v) => {
        if v.to_string() == ";" {
          let filename = format!("../testdata/{}.in", basename.as_ref().unwrap().to_string());
          let w0 = want0.unwrap();
          let w1 = want1.unwrap();
          tests_output.extend(quote! {
            #[test]
            fn #basename() {
              let want = (#w0.to_string(), #w1.to_string());
              let got = super::doit(&read_lines(#filename));
              assert_eq!(got, want);
            }
          });
          want0 = None;
          want1 = None;
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
