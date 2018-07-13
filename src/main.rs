mod base64;
mod hex_io;
mod xor;

fn main() {
    println!("Enter hex string:");
    match hex_io::read_hex_string() {
        Ok(string) => {

        }
        Err(descr) => {
            println!("Error reading string: {}", descr);
        }
    }
}
