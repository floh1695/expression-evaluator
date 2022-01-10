use std::error::Error;

use crate::{token::Token, utility::debug_println, word::Word};

#[derive(Clone, Debug)]
pub enum Ast {
  Number(f64),
  Unary(String, Box<Ast>),
  Binary(String, Box<Ast>, Box<Ast>),
}

impl Ast {
  pub fn evaluate_expression(expression: String) -> Result<f64, Box<dyn Error>> {
    let words = Word::scan(expression)?;
    debug_println(format!("    words:  {:?}", words).to_string());

    let tokens = Token::parse(words)?;
    debug_println(format!("    tokens: {:?}", tokens).to_string());

    let ast = Ast::parse(tokens)?;
    debug_println(format!("    ast:    {:?}", ast).to_string());

    let result = ast.evaluate()?;

    Ok(result)
  }

  pub fn evaluate(&self) -> Result<f64, Box<dyn Error>> {
    match self {
      Ast::Number(number) => Ok(*number),
      Ast::Unary(operator, right) => {
        let right = right.evaluate()?;

        match operator.as_str() {
          "-" => Ok(-right),
          _   => Err(format!("Unknown unary operator: {}", operator).into()),
        }
      },
      Ast::Binary(operator, left, right) => {
        let left = left.evaluate()?;
        let right = right.evaluate()?;

        match operator.as_str() {
          "+"  => Ok(left + right),
          "-"  => Ok(left - right),
          "*"  => Ok(left * right),
          "/"  => Ok(left / right),
          "**" => Ok(left.powf(right)),
          _    => Err(format!("Unknown binary operator: {}", operator).into()),
        }
      },
    }
  }

  pub fn parse(tokens: Vec<Token>) -> Result<Ast, Box<dyn Error>> {
    let mut ast = None;

    let mut forward = tokens.clone();
    while !forward.is_empty() {
      let next = forward.remove(0);
      
      match next.clone() {
        value if value.is_value() => {
          let new = Ast::parse_single(value)?;

          match ast {
            Some(_) => return Err("Cannot have multiple values in a row".into()),
            None => ast = Some(new),
          }
        },
        Token::Operator(operator) => {
          if forward.is_empty() {
            return Err("Cannot use operator without right value".into());
          }

          let right = forward.remove(0);

          let right = match right {
            value if value.is_value() => {
              Ast::parse_single(value)?
            },
            Token::Operator(operator) => {
              if forward.is_empty() {
                return Err("Cannot use operator without right value".into());
              }

              let right = forward.remove(0);
              let right = Ast::parse_single(right)?;
              let right = Box::new(right);

              Ast::Unary(operator, right)
            },
            _ => return Err(format!("Unmatched token type: {:?}", next.clone()).into()),
          };

          let right = Box::new(right);

          let new = match ast {
            Some(left) => Ast::Binary(operator, Box::new(left), right),
            None => Ast::Unary(operator, right),
          };

          ast = Some(new);
        },
        _ => return Err(format!("Unmatched token type: {:?}", next.clone()).into()),
      }
    }

    let ast = ast.ok_or_else(|| "No tree constructed")?;

    Ok(ast)
  }

  fn parse_single(token: Token) -> Result<Ast, Box<dyn Error>> {
    match token {
      Token::Number(number) => Ok(Ast::Number(number)),
      Token::Group(group) => Ast::parse(group),
      _ => Err(format!("Cannot process {:?} as value token", token).into()),
    }
  }
}
