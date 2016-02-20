extern crate grabinput;

use std::io::{Cursor, Read};

mod funny_read;
use funny_read::Atbash;

fn main() {
  let mut input = Cursor::new(grabinput::all(std::env::args().nth(1))).atbash();
  let mut buf = String::new();

  input.read_to_string(&mut buf).ok();

  println!("{}", buf);
}
