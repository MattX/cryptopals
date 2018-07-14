mod base64;
mod hex_io;
mod xor;
mod evaluate;
mod xor_guesser;

use std::io;

fn main() {
    println!("Enter filename:");
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(string) => {
            println!("{}", xor_guesser::multi_xor(&input.trim()));
        }
        Err(descr) => {
            println!("Error reading string: {}", descr);
        }
    }
}
