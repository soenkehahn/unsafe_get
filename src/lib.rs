extern crate proc_macro;

use core::iter::FromIterator;
use core::str::FromStr;
use proc_macro::*;

fn split_by(separators: Vec<char>, tokens: TokenStream) -> Vec<Vec<TokenTree>> {
    let mut result = vec![];
    let mut acc = vec![];
    for token in tokens {
        if let TokenTree::Punct(punct) = token {
            if separators.contains(&punct.as_char()) {
                result.push(acc);
                acc = vec![];
            } else {
                acc.push(TokenTree::Punct(punct))
            }
        } else {
            acc.push(token)
        }
    }
    if !acc.is_empty() {
        result.push(acc);
    }
    result
}

struct Arguments {
    expr: String,
    constructor: String,
    member: String,
}

impl Arguments {
    fn new(input: TokenStream) -> Arguments {
        fn next_argument(arguments: &mut dyn Iterator<Item = Vec<TokenTree>>) -> String {
            TokenStream::from_iter(arguments.next().unwrap()).to_string()
        }
        let mut arguments = split_by(vec![',', '.'], input).into_iter();
        Arguments {
            expr: next_argument(&mut arguments),
            constructor: next_argument(&mut arguments),
            member: next_argument(&mut arguments),
        }
    }
}

#[proc_macro]
pub fn get(input: TokenStream) -> TokenStream {
    let arguments = Arguments::new(input);
    TokenStream::from_str(&format!(
        "
            match ({}) {{
                {} {{ {}, .. }} => {},
                x => panic!(\"get!: expected enum constructor: {}, got {{:?}}\", x),
            }}
        ",
        arguments.expr,
        arguments.constructor,
        arguments.member,
        arguments.member,
        remove_spaces(&arguments.constructor),
    ))
    .unwrap()
}

fn remove_spaces(s: &str) -> String {
    s.chars().filter(|char| *char != ' ').collect()
}
