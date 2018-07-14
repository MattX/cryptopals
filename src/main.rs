mod base64;
mod hex_io;
mod xor;
mod single_xor;
mod evaluate;
mod multi_xor;

use std::io;

fn main() {
    println!("Enter filename:");
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(string) => {
            println!("{}", multi_xor::multi_xor(&input.trim()));
        }
        Err(descr) => {
            println!("Error reading string: {}", descr);
        }
    }
}
