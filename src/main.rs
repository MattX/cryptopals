extern crate base64;
extern crate openssl;

mod aes;
mod evaluate;
mod hamming;
mod hex_io;
mod pkcs;
mod util;
mod xor;
mod xor_guesser;

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
