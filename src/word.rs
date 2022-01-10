use std::error::Error;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Word {
  Number(String),
  Operator(String),
  OpenGroup,
  CloseGroup,
}

lazy_static! {
  static ref WORDS_PATTERN: Regex = Regex::new(
    r"((?:\-?\d+(?:\.\d+)?)|\(|\)|\+|\-|\*{1,2}|/)"
  ).unwrap();

  static ref NUMBER_PATTERN: Regex = Regex::new(
    r"\-?\d+(?:\.\d+)?"
  ).unwrap();

  static ref OPERATOR_PATTERN: Regex = Regex::new(
    r"\+|\-|\*{1,2}|/"
  ).unwrap();

  static ref GROUPING_OPEN_PATTERN: Regex = Regex::new(
    r"\("
  ).unwrap();

  static ref GROUPING_CLOSE_PATTERN: Regex = Regex::new(
    r"\)"
  ).unwrap();
}

impl Word {
  pub fn scan(expression: String) -> Result<Vec<Word>, Box<dyn Error>> {
    WORDS_PATTERN
      .find_iter(expression.as_str())
      .map(|m| m.as_str().to_string())
      .map(|w| Word::parse_single(w))
      .collect()
  }

  fn parse_single(raw: String) -> Result<Word, Box<dyn Error>> {
    let word = match raw.as_str() {
      number if NUMBER_PATTERN.is_match(number) => {
        Some(Word::Number(number.to_string()))
      },
      operator if OPERATOR_PATTERN.is_match(operator) => {
        Some(Word::Operator(operator.to_string()))
      },
      "(" => Some(Word::OpenGroup),
      ")" => Some(Word::CloseGroup),
      _ => None,
    };

    let word = word.ok_or_else(|| format!("Could not parse raw word: {}", raw))?;

    Ok(word)
  }
}
