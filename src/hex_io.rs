//! I/O of hexadecimal/base64 values.

use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Read};
use std::ops::Deref;

/// Prints an array of bytes as an hex string
pub fn print_hex_string(data: &[u8]) -> () {
  println!("{}", hex_to_string(data));
}

/// Turns an array of bytes into its string representation in hex
pub fn hex_to_string(data: &[u8]) -> String {
  data.into_iter()
      .map(|byte| format!("{:x}", byte))
      .collect::<Vec<String>>().join("")
}

/// Reads a line of hex data from stdin and turns it into an array of bytes
pub fn read_hex_string() -> Result<Box<[u8]>, String> {
  let mut input: String = String::new();
  io::stdin().read_line(&mut input).map_err(|err| err.to_string())?;
  hex_string_to_bytes(input.trim())
}

/// Reads a sequence of hex strings from a file
pub fn read_hex_file(filename: &str) -> Result<Vec<Box<[u8]>>, String> {
  let mut file = File::open(filename).map_err(|e| e.to_string())?;
  BufReader::new(file).lines()
      .map(|lr| lr.map_err(|e| e.to_string()).and_then(|l| hex_string_to_bytes(&l)))
      .collect()
}

/// Turns a string representing hex values into an array of bytes
pub fn hex_string_to_bytes(hex: &str) -> Result<Box<[u8]>, String> {
  let count = hex.chars().count();
  match count % 2 {
    0 => {
      let mut result = vec![0; count / 2].into_boxed_slice();
      let maybe_chars: Result<Vec<u8>, String> = hex.chars()
          .filter(|c| !c.is_ascii_whitespace())
          .map(|c| c.to_digit(16)
              .map(|n| n as u8)
              .ok_or(format!("Invalid digit {}", c)))
          .collect();
      let chars = maybe_chars?;
      for i in 0..count / 2 {
        result[i] = chars[2 * i] * 16 + chars[2 * i + 1];
      }
      Ok(result)
    }
    _ => Err(format!("Wrong parity of half-bytes in hex string: {}", count))
  }
}

pub fn load_b64_from_file(filename: &str) -> Box<[u8]> {
  let mut test_data_file = File::open(filename).unwrap();
  let mut test_data_string = String::new();
  test_data_file.read_to_string(&mut test_data_string).unwrap();
  test_data_string.retain(|c| !c.is_whitespace());

  base64::decode(test_data_string.as_bytes()).unwrap().into_boxed_slice()
}

#[cfg(test)]
mod tests {
  use base64;
  use super::*;

  const EXAMPLE_STRING: &'static str =
    "49276d206b696c6c696e6720796f757220627261696e206c\
    696b65206120706f69736f6e6f7573206d757368726f6f6d";

  #[test]
  fn convert_example_string() {
    assert_eq!(
      hex_string_to_bytes(EXAMPLE_STRING)
          .unwrap().deref(),
      &[73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98,
        114, 97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111,
        110, 111, 117, 115, 32, 109, 117, 115, 104, 114, 111, 111, 109][..]
    )
  }

  #[test]
  fn challenge_1() {
    assert_eq!(
      base64::encode(&hex_string_to_bytes(EXAMPLE_STRING).unwrap()),
      "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    )
  }
}
