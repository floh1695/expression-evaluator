pub enum Request {
  Test,
  Expressions(Vec<String>),
}

const TESTS_FLAG: &'static str = "--tests";

impl Request {
  pub fn parse(raws: Vec<String>) -> Vec<Request> {
    let mut raws = raws.clone();
    let raws: Vec<String> = raws
      .drain(1..)
      .collect();

    let mut expressions: Vec<String> = vec![];
    let mut run_tests = false;
    for raw in &raws {
      match &raw[..] {
        TESTS_FLAG => run_tests = true,
        _ => expressions.push(raw.clone()),
      }
    }

    let mut inputs = vec![];
    
    if run_tests {
      inputs.push(Request::Test);
    }

    if !expressions.is_empty() {
      inputs.push(Request::Expressions(expressions));
    }

    inputs
  }
}

