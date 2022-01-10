use std::collections::HashMap;

use crate::scanner::ProtoToken;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Number(f64),
  Operator(String),
  Group(Vec<Token>)
}

pub fn parse(protos: Vec<ProtoToken>) -> Vec<Token> {
  let mut tokens: Vec<Token> = vec![];

  let mut grouping_depth = 0;
  let mut grouping_proto_tokens: Vec<ProtoToken> = vec![];

  for proto in protos {
    if grouping_depth == 0 {
      match proto {
        ProtoToken::Unknown(raw) => {
          panic!("Cannot process unknown token: '{:?}'", raw);
        },
        ProtoToken::Number(n) => {
          tokens.push(Token::Number(n));
        },
        ProtoToken::Operator(op) => {
          tokens.push(Token::Operator(op));
        },
        ProtoToken::OpenGroup => {
          grouping_depth += 1;
        },
        ProtoToken::CloseGroup => {
          panic!("Group closing has no opening");
        },
      }
    } else if grouping_depth > 0 {
      match proto {
        ProtoToken::OpenGroup => grouping_depth += 1,
        ProtoToken::CloseGroup => grouping_depth -= 1,
        _ => {},
      }

      if grouping_depth == 0 {
        let grouping_tokens = parse(grouping_proto_tokens.clone());
        tokens.push(Token::Group(grouping_tokens));

        grouping_proto_tokens.clear();
      } else {
        grouping_proto_tokens.push(proto);
      }
    } else {
      panic!("Unknown depth issue");
    }
  }

  tokens
}

// fn rewrite_negation(tokens: Vec<Token>) -> Vec<Token> {
//   let mut forward = tokens.clone();
//   let mut processed: Vec<Token> = vec![];
//   // while !forward.is_empty() {

//   // }

//   processed
// }

// pub fn with_precedence(tokens: Vec<Token>, mapping: HashMap<i32, Vec<String>>) -> Vec<Token> {
//   let mut keys: Vec<i32> = mapping.keys()
//     .cloned()
//     .collect();
//   keys.sort();
//   let keys: Vec<i32> = keys.into_iter()
//     .rev()
//     .collect();
//   let lowest = keys.last().unwrap();

//   let mut new_tokens = tokens.clone();
//   for key in &keys {
//     let operators = mapping.get(key)
//       .unwrap();

//     if key != lowest {
//       new_tokens = with_single_precedence(new_tokens, operators.to_owned());
//     }
//   }

//   tokens
// }

// fn with_single_precedence(tokens: Vec<Token>, operators: Vec<String>) -> Vec<Token> {
//   let mut forward = tokens.clone();
//   let mut processed: Vec<Token> = vec![];
//   // while !forward.is_empty() {

//   // }

//   processed
// }
