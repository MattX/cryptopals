mod base64;
mod hex_io;
use std::str;

fn main() {
    println!("Enter hex string:");
    match hex_io::read_hex_string() {
        Ok(string) => {
            str::from_utf8(&base64::to_base64(&string)).map(|s| println!("{}", s));
        }
        Err(descr) => {
            println!("Error reading string: {}", descr);
        }
    }
}
