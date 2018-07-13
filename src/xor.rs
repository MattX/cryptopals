pub fn xor(a: &[u8], b: &[u8]) -> Box<[u8]> {
    if a.len() != b.len() {
        panic!("Xoring vectors of different lengths");
    }

    let result: Vec<u8> = a.into_iter()
        .zip(b)
        .map(|(x, y)| x ^ y)
        .collect();

    result.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_io;

    #[test]
    fn convert_example_string() {
        assert_eq!(xor(
            &hex_io::hex_string_to_bytes("1c0111001f010100061a024b53535009181c").unwrap(),
            &hex_io::hex_string_to_bytes("686974207468652062756c6c277320657965").unwrap()
        ),
        hex_io::hex_string_to_bytes("746865206b696420646f6e277420706c6179").unwrap());
    }
}
