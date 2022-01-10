use std::collections::HashMap;

use crate::{lexer::Token, operator};

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
  Empty,
  Number(f64),
  Unary(String, Box<Ast>),
  Binary(String, Box<Ast>, Box<Ast>),
}

impl Ast {
  pub fn eval(&self) -> f64 {
    match self {
      Ast::Empty => 0.0,
      Ast::Number(n) => n.to_owned(),
      Ast::Unary(op, a) => {
        let operator = operator::get_unary(op.to_owned())
          .unwrap();
        operator(a.eval())
      },
      Ast::Binary(op, a, b) => {
        let operator = operator::get_binary(op.to_owned())
          .unwrap();
        operator(a.eval(), b.eval())
      },
    }
  }
}

pub fn parse(tokens: Vec<Token>, precedences: HashMap<i32, Vec<String>>) -> Box<Ast> {
  let mut forward = tokens.clone();
  let mut last: Option<Box<Ast>> = None;
  let mut ast = Box::new(Ast::Empty);
  while !forward.is_empty() {
    let current = forward.remove(0);
    
    match current {
      Token::Operator(op) => {
        let next = forward.remove(0);
        let next = parse_single(next);

        match *ast {
          Ast::Empty => {
            let new_ast = match last.clone() {
              Some(a) => Ast::Binary(op, a, next),
              None => Ast::Unary(op, next),
            };

            ast = Box::new(new_ast);
          },
          _ => {
            ast = Box::new(Ast::Binary(op, ast, next));
          },
        }
      },
      _ => {
        match last {
          Some(_) => panic!("Cannot process values in a row"),
          None => last = Some(parse_single(current)),
        }
      },
    }
  }

  ast
}

fn parse_single(token: Token) -> Box<Ast> {
  match token {
    Token::Number(n) => Box::new(Ast::Number(n)),
    Token::Group(ts) => parse(ts),
    _ => Box::new(Ast::Empty),
  }
}
