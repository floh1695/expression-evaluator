use std::error::Error;

use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
  Empty,
  Number(f64),
  Unary(String, Box<Ast>),
  Binary(String, Box<Ast>, Box<Ast>),
}

impl Ast {
  pub fn evaluate_expression(expression: String) -> Result<f64, Box<dyn Error>> {
    let tokens = Token::scan(expression)?;
    let ast = Ast::parse(tokens)?;
    let answer = ast.evaluate()?;

    Ok(answer)
  }

  pub fn parse(tokens: Vec<Token>) -> Result<Ast, Box<dyn Error>> {
    let mut forward = tokens.clone();
    let mut last = Ast::Empty;
    let mut ast = Ast::Empty;

    while !forward.is_empty() {
      let current = forward.remove(0);

      match current {
        Token::Empty => {},
        Token::Number(n) => {
          match last {
            Ast::Empty => last = Ast::Number(n),
            Ast
          }
        },
        Token::Operator(op) => {},
        Token::Group(ts) => {},
        Token::Error(e) => {
          return Err(e.into());
        },
        _ => {
          return Err(format!("Unexpected token: {:?}", current).into());
        },
      }
    }

    Ok(Ast::Empty)
  }

  pub fn evaluate(&self) -> Result<f64, Box<dyn Error>> {
    Ok(0.0)
  }
}

    //   match (
    //     ast.clone(),
    //     last.clone(),
    //     current.clone()
    //   ) {
    //     ( Ast::Empty,
    //       Ast::Empty,
    //       Token::Number(x)
    //     ) => {
    //       ast = Ast::Number(x);

    //       last = 
    //     },
    //     // (
    //     //   Ast::Number(_),
    //     //   Ast::
    //     // )
    //     _ => {
    //       return Err(format!("Unknown parser state: {:?}, {:?}, {:?}", ast.clone(), last.clone(), current.clone()).into());
    //     },
    //     // (Token::Empty, _, _) => return Err("Cannot parse an empty token"),
    //     // (Token::Number(_), Ast::Number(_), _) => 
    //   }
    // }
