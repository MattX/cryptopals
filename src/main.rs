extern crate num;
extern crate base64;

mod hex_io;
mod xor;
mod evaluate;
mod xor_guesser;
mod hamming;

use std::io;

fn main() {
  print!("Enter filename:");
  let mut input: String = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {
      let encrypted = hex_io::read_hex_file(&input).unwrap();

    }
    Err(descr) => {
      println!("Error reading string: {}", descr);
    }
  }
}
