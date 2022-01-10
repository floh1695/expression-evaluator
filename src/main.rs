use std::{env,error::Error};

use ast::Ast;
use request::Request;
use tests::TestDescription;
use utility::debug_println;

mod ast;
mod configuration;
mod request;
mod tests;
mod token;
mod utility;
mod word;

fn main() -> Result<(), Box<dyn Error>> {
  let arguments: Vec<String> = env::args().collect();
  let requests = Request::parse(arguments);

  for request in requests {
    process_request(request);
  }

  Ok(())
}

fn process_request(request: Request) {
  match request {
    Request::Test => {
      let tests = vec![
        TestDescription::new(
          "Number".to_string(),
          "1".to_string(),
          1.0
        ),
        TestDescription::new(
          "Negative".to_string(),
          "-2".to_string(),
          -2.0
        ),
        TestDescription::new(
          "Group number".to_string(),
          "(3)".to_string(),
          3.0
        ),
        TestDescription::new(
          "Group negative".to_string(),
          "-(4)".to_string(),
          -4.0
        ),
        TestDescription::new(
          "Addition".to_string(),
          "1 + 2.5".to_string(),
          3.5
        ),
        TestDescription::new(
          "Subtraction".to_string(),
          "5.8 - 3.2".to_string(),
          2.6
        ),
        TestDescription::new(
          "Multiplication".to_string(),
          "4 * 5".to_string(),
          20.0
        ),
        TestDescription::new(
          "Division".to_string(),
          "4 / 5".to_string(),
          0.8
        ),
        TestDescription::new(
          "Power".to_string(),
          "2 ** 4".to_string(),
          16.0
        ),
        TestDescription::new(
          "Negation across multiplication".to_string(),
          "-(1 + 2) * -(16 / 4)".to_string(),
          12.0
        ),
        // TestDescription::new(
        //   "".to_string(),
        //   "".to_string(),
        //   0.0
        // ),
      ];

      for test in tests {
        debug_println(format!("{:?}", test.run()));
        debug_println("".to_string());
      }
    },
    Request::Expressions(expressions) => {
      for expression in &expressions {
        process_expression(expression.clone());
      }
    },
  }
}

fn process_expression(expression: String) {
  debug_println(format!("Processing expression: {:?}", expression.clone()).to_string());

  let results = Ast::evaluate_expression(expression);
  debug_println(format!("-> {:?}", results).to_string());

  debug_println("".to_string());
}
