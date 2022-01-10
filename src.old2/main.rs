use std::{env,error::Error};

use ast::Ast;
use request::Request;

mod ast;
mod configuration;
mod request;
mod tests;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
  let arguments: Vec<String> = env::args().collect();
  let requests = Request::parse(arguments);

  for request in requests {
    process_request(request)?;
  }

  Ok(())
}

fn process_request(request: Request) -> Result<(), Box<dyn Error>> {
  match request {
    Request::Test => {},
    Request::Expressions(expressions) => {
      for expression in &expressions {
        process_expression(expression.clone())?;
      }
    },
  }
  
  Ok(())
}

fn process_expression(expression: String) -> Result<(), Box<dyn Error>> {
  if configuration::DEBUG {
    println!("Processing expression: {:?}", expression.clone());
  }

  let results = Ast::evaluate_expression(expression.clone());
  match results {
    Ok(answer) => {
      if configuration::DEBUG {
        println!("-> {:?}", answer);
      }
    },
    Err(error) => println!("{:?}", error),
  }

  if configuration::DEBUG {
    println!();
  }

  Ok(())
}
