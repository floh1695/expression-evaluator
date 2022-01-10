use std::error::Error;

use lazy_static::lazy_static;
use regex::Regex;

use crate::configuration;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Empty,
  Number(f64),
  Symbol(String),
  Operator(String),

  GroupOpening,
  GroupClosing,
  Group(Vec<Token>),

  Skip,
  Error(String),
}

lazy_static! {
  static ref WORDS_PATTERN: Regex = Regex::new(
    r"(?:\-?\d+(?:\.\d+)?)|(?:[a-zA-Z_]\w+)|\(|\)|\+|\-{1,2}|\*{1,2}|/"
  ).unwrap();

  static ref NUMBER_PATTERN: Regex = Regex::new(
    r"\-?\d+(?:\.\d+)?"
  ).unwrap();

  static ref SYMBOL_PATTERN: Regex = Regex::new(
    r"[a-zA-Z_]\w+"
  ).unwrap();

  static ref OPERATOR_PATTERN: Regex = Regex::new(
    r"\+|\-{1,2}|\*{1,2}|/"
  ).unwrap();

  static ref GROUPING_OPEN_PATTERN: Regex = Regex::new(
    r"\("
  ).unwrap();

  static ref GROUPING_CLOSE_PATTERN: Regex = Regex::new(
    r"\)"
  ).unwrap();
}

impl Token {
  pub fn parse(words: Vec<String>) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = vec![];
    let mut last = Token::Empty;
    let mut push_token = |t: Token| {
      last = t.clone();
      tokens.push(t);
    };

    let mut to_process = words.clone();
    let mut depth = 0;
    let mut group_words = vec![];
    while !to_process.is_empty() {
      // if configuration::DEBUG {
      //   println!("{:?}: {:?}", "to_process ", to_process);
      //   println!("{:?}: {:?}", "depth      ", depth);
      //   println!("{:?}: {:?}", "group_words", group_words.clone());
      //   println!();
      // }

      let word = to_process.remove(0);
      let word_str = &word[..];

      if depth == 0 {
        if GROUPING_OPEN_PATTERN.is_match(word_str) {
          depth += 1;

          continue;
        } else if GROUPING_CLOSE_PATTERN.is_match(word_str) {
          return Err("Attempting to close an unopened group".into());
        } else {
          let token = Token::parse_single(word)?;
          push_token(token);
        }
      } else if depth > 0 {
        if GROUPING_OPEN_PATTERN.is_match(word_str) {
          depth += 1;
        } else if GROUPING_CLOSE_PATTERN.is_match(word_str) {
          depth -= 1;

          if depth == 0 {
            let group_tokens = Token::parse(group_words.clone())?;
            group_words.clear();
            let token = Token::Group(group_tokens);

            push_token(token);

            continue;
          }
        }

        group_words.push(word);
      } else {
        return Err("Somehow the depth got to a negative value".into());
      }
    }

    Ok(tokens)
  }

  pub fn parse_single(word: String) -> Result<Token, Box<dyn Error>> {
    let word_str = &word[..];

    let token =
      if NUMBER_PATTERN.is_match(word_str) {
        let number = word.parse::<f64>()?;
        Token::Number(number)
      } else if SYMBOL_PATTERN.is_match(word_str) {
        Token::Symbol(word)
      } else if OPERATOR_PATTERN.is_match(word_str) {
        Token::Operator(word)
      } else if GROUPING_OPEN_PATTERN.is_match(word_str) {
        Token::GroupOpening
      } else if GROUPING_CLOSE_PATTERN.is_match(word_str) {
        Token::GroupClosing
      } else {
        Token::Error("Word matched no token patterns".to_string())
      };

    Ok(token)
  }

  pub fn scan(expression: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let words = scan_words(expression);
    let tokens = Token::parse(words)?;

    if configuration::DEBUG {
      println!("{:?}: {:?}", "tokens", tokens.clone());
    }

    Ok(tokens)
  }

  pub fn is_empty(&self) -> bool {
    self.to_owned() == Token::Empty
  }

  pub fn is_value(&self) -> bool {
    match self {
      Token::Number(_) => true,
      Token::Symbol(_) => true,
      _ => false,
    }
  }
}

fn scan_words(expression: String) -> Vec<String> {
  let words = WORDS_PATTERN
    .find_iter(&expression[..])
    .map(|m| m.as_str().to_string())
    .collect();

  words
}
