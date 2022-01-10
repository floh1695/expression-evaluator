use std::error::Error;

use float_cmp::{ApproxEq,F64Margin};

use crate::ast::Ast;

pub struct TestDescription {
  name: String,
  expression: String,
  expected: f64
}

impl TestDescription {
  pub fn new(name: String, expression: String, expected: f64) -> TestDescription {
    TestDescription {
      name,
      expression,
      expected
    }
  }

  pub fn run(&self) -> Result<(), Box<dyn Error>> {
    println!("Running test: {}", self.name);
    println!("  expression: {}", self.expression);
    println!("  expected:   {:?}", self.expected);

    let result = Ast::evaluate_expression(self.expression.clone())?;
    let passed = result.approx_eq(self.expected, F64Margin::default());

    if passed {
      println!("  Passed with {:?}!", result);

      Ok(())
    } else {
      Err(format!("Unexpected result: {:?}", result).into())
    }
  }
}
