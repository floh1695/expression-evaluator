use crate::configuration;

/*
  TODO: Not very important, but it would be nice to change this into a macro:

  `debug_println!()`                        -> `println!()`
  `debug_println!("Number: {:?}", 12345)`   -> `println!("Number: {:?}", 12345)`

  And I would expect it to work for any number of arguments.

  Note: May need to use `macro_const!` to reference `configuration::DEBUG`
*/
pub fn debug_println(string: String) {
  if configuration::DEBUG {
    println!("{}", string);
  }
}
