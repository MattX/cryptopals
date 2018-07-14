use std::io;

pub fn hex_to_string(data: &[u8]) -> String {
    data.into_iter()
        .map(|byte| format!("{:x}", byte))
        .collect::<Vec<String>>().join("")
}

pub fn read_hex_string() -> Result<Box<[u8]>, String> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).map_err(|err| err.to_string())?;
    hex_string_to_bytes(input.trim())
}

pub fn hex_string_to_bytes(hex: &str) -> Result<Box<[u8]>, String> {
    let count = hex.chars().count();
    match count % 2 {
        0 => {
            let mut result = vec![0; count / 2].into_boxed_slice();
            let chars: Vec<u8> = hex.chars().map(|c|
                c.to_digit(16).unwrap() as u8
            ).collect();
            for i in 0..count / 2 {
                result[i] = chars[2*i] * 16 + chars[2*i + 1];
            }
            Ok(result)
        },
        _ => Err(format!("Wrong parity of half-bytes in hex string: {}", count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn convert_example_string() {
        assert_eq!(
            hex_string_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
                .unwrap().deref(),
            &[73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114, 32, 98,
                114, 97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105, 115, 111,
                110, 111, 117, 115, 32, 109, 117, 115, 104, 114, 111, 111, 109][..]
        )
    }
}
