use std::{collections::HashMap, env};

mod lexer;
mod operator;
mod parser;
mod scanner;

/* Scratchpad

        /* This feels like it should be parsed at the AST level */
        // if word_str == "-" && !last.is_value() {
        //   push_token(Token::Operator("--".to_string()));

        //   continue;
        // }

      
      // let token = if NUMBER_PATTERN.is_match(word_str) {
      //   let number = word.parse::<f64>()?;

      //   Token::Number(number)
      // } else if SYMBOL_PATTERN.is_match(word_str) {
      //   Token::Symbol(word)
      // } else if OPERATOR_PATTERN.is_match(word_str) {
      //   if word_str == "-"
      //       && tokens.last()
      //           .map(|t: &Token| !t.is_value_token())
      //           .unwrap_or(true) {
      //     Token::Operator("--".to_string())
      //   } else {
      //     Token::Operator(word)
      //   }
      // } else if GROUPING_OPEN_PATTERN.is_match(word_str) {
      //   depth += 1;

      //   Token::Skip
      // } else if GROUPING_CLOSE_PATTERN.is_match(word_str) {
      //   depth -= 1;

      //   if depth < 0 {
      //     Token::Error("Reached a negative grouping depth".to_string())
      //   } else if depth == 0 {
      //     let token = Token::Group(group_words.clone());
      //     group_words.clear();

      //     token
      //   } else {
      //     Token::Skip
      //   }
      // } else {
      //   Token::Error("Unknown word!".to_string())
      // };

      // if let Token::Error(error) = token {
      //   return Err(error.into());
      // } else if token == Token::Skip {
      //   continue;
      // } else if depth == 0 {
        
      // }


*/

const DEBUG: bool = true;

fn main() {
  let expressions = get_expressions();

  if DEBUG {
    println!("Inputs:");
    for e in &expressions {
      println!("  {:?}", e);
    }
    println!("");
  }

  for e in &expressions {
    process_expression(e.to_string());
  }
}

fn get_expressions() -> Vec<String> {
  let mut arguments: Vec<String> = env::args()
    .collect();
  arguments
    .drain(1..)
    .collect()
}

fn process_expression(expression: String) {
  if DEBUG {
    println!("Processing: {:?}", expression);
  }

  let protos = scanner::scan(expression);
  
  let tokens = lexer::parse(protos);
  let precedences = get_precedences();
  // let tokens = lexer::with_precedence(tokens, precedences);
  
  let ast = parser::parse(
    tokens.clone(),
  );
  let result = ast.eval();

  if DEBUG {
    println!("  -> {:?}", result);
  }
}

fn get_precedences() -> HashMap<i32, Vec<String>> {
  let mut mapping = HashMap::new();

  mapping.insert(2, 
    vec![
      "--".to_string()
    ]);
  mapping.insert(1, 
    vec![
      "*".to_string(),
      "/".to_string()
    ]);
  mapping.insert(0,
    vec![
      "+".to_string(),
      "-".to_string()
    ]);

  mapping
}
