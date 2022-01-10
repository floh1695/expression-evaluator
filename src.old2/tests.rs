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
}
