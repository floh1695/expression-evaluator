use regex::Regex;

#[derive(Clone, Debug)]
pub enum ProtoToken {
  Unknown(String),
  Number(f64),
  Operator(String),
  OpenGroup,
  CloseGroup,
}

pub fn scan(expression: String) -> Vec<ProtoToken> {
  let raw_tokens = get_raw_tokens(expression);
  raw_tokens.into_iter()
    .map(raw_to_token)
    .collect()
}

fn get_raw_tokens(expression: String) -> Vec<String> {
  let token_re = Regex::new(r"((?:\-?\d+)|\w+|\+|\-|\*|/|\(|\))")
    .unwrap();
  let matches = token_re.find_iter(&expression[..]);
  matches
    .map(|m| m.as_str().to_string())
    .collect()
}

fn raw_to_token(raw: String) -> ProtoToken {
  let number_re = Regex::new(r"\-?\d+")
    .unwrap();
  let operator_re = Regex::new(r"\+|\-|\*|/")
    .unwrap();
  let open_group_re = Regex::new(r"\(")
    .unwrap();
  let close_group_re = Regex::new(r"\)")
    .unwrap();

  let raw_str = &raw[..];
  if number_re.is_match(raw_str) {
    let number = raw.parse::<f64>().unwrap();
    ProtoToken::Number(number)
  } else if operator_re.is_match(raw_str) {
    ProtoToken::Operator(raw)
  } else if open_group_re.is_match(raw_str) {
    ProtoToken::OpenGroup
  } else if close_group_re.is_match(raw_str) {
    ProtoToken::CloseGroup
  } else {
    ProtoToken::Unknown(raw)
  }
}
