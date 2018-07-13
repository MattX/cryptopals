mod base64;
mod hex_io;
mod xor;
mod single_xor;

fn main() {
    println!("Enter hex string:");
    match hex_io::read_hex_string() {
        Ok(string) => {
            println!("{}", single_xor::find_best_xor(&string));
        }
        Err(descr) => {
            println!("Error reading string: {}", descr);
        }
    }
}
