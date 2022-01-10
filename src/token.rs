use std::error::Error;

use crate::word::Word;

#[derive(Clone, Debug)]
pub enum Token {
  Number(f64),
  Operator(String),
  Group(Vec<Token>),
}

impl Token {
  pub fn parse(words: Vec<Word>) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = vec![];

    let mut forward = words.clone();
    while !forward.is_empty() {
      let token = Token::parse_next(&mut forward)?;
      tokens.push(token);
    }

    Ok(tokens)
  }

  fn parse_next(forward: &mut Vec<Word>) -> Result<Token, Box<dyn Error>> {
    let word = forward.remove(0);
    
    match word {
      Word::Number(raw) => {
        let number = raw.parse::<f64>()?;
        let token = Token::Number(number);

        Ok(token)
      },
      Word::Operator(operator) => {
        let token = Token::Operator(operator);

        Ok(token)
      },
      Word::OpenGroup => {
        let token = Token::parse_group(forward)?;

        Ok(token)
      },
      Word::CloseGroup => {
        Err("Attempting to close unopened grouping".into())
      }
    }
  }

  fn parse_group(forward: &mut Vec<Word>) -> Result<Token, Box<dyn Error>> {
    let mut depth = 1;
    let mut group = vec![];

    while depth > 0 {
      let next = forward.remove(0);

      match next {
        Word::OpenGroup => depth += 1,
        Word::CloseGroup => depth -= 1,
        _ => {},
      }

      if depth == 0 {
        continue;
      }

      group.push(next);
    }

    let group = Token::parse(group)?;
    let token = Token::Group(group);

    Ok(token)
  }

  pub fn is_value(&self) -> bool {
    match self {
      Token::Operator(_) => false,
      _                  => true,
    }
  }
}
