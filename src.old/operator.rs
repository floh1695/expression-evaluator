fn negative(x: f64) -> f64 {
  -x
}

pub fn get_unary(op: String) -> Option<Box<dyn Fn(f64) -> f64>> {
  match &op[..] {
    "-" => Some(Box::new(negative)),
    "--" => Some(Box::new(negative)),
    _ => None
  }
}

fn addition(x: f64, y: f64) -> f64 {
  x + y
}

fn subtraction(x: f64, y: f64) -> f64 {
  x - y
}

fn multiplication(x: f64, y: f64) -> f64 {
  x * y
}

fn division(x: f64, y: f64) -> f64 {
  x / y
}

pub fn get_binary(op: String) -> Option<Box<dyn Fn(f64, f64) -> f64>> {
  match &op[..] {
    "+" => Some(Box::new(addition)),
    "-" => Some(Box::new(subtraction)),
    "*" => Some(Box::new(multiplication)),
    "/" => Some(Box::new(division)),
    _ => None
  }
}
