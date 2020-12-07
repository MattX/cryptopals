//! Functions for XORing byte arrays

pub fn xor(a: &[u8], b: &[u8]) -> Box<[u8]> {
    if a.len() != b.len() {
        panic!("Xoring vectors of different lengths");
    }

    let result: Vec<u8> = a.iter().zip(b).map(|(x, y)| x ^ y).collect();

    result.into_boxed_slice()
}

pub fn rep_key_xor(input: &[u8], key: &[u8]) -> Box<[u8]> {
    let extended_key: Vec<u8> = key.iter().cycle().take(input.len()).copied().collect();

    xor(input, &extended_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex_io::*;

    #[test]
    fn challenge_2() {
        assert_eq!(
            xor(
                &hex_string_to_bytes("1c0111001f010100061a024b53535009181c").unwrap(),
                &hex_string_to_bytes("686974207468652062756c6c277320657965").unwrap()
            ),
            hex_string_to_bytes("746865206b696420646f6e277420706c6179").unwrap()
        );
    }

    const CHALLENGE_5_PLAINTEXT: &'static str = "\
    Burning 'em, if you ain't quick and nimble\n\
    I go crazy when I hear a cymbal";

    const CHALLENGE_5_KEY: &'static str = "ICE";

    const CHALLENGE_5_RESULT: &'static str = "\
    0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
    a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    #[test]
    fn challenge_5() {
        assert_eq!(
            &rep_key_xor(CHALLENGE_5_PLAINTEXT.as_bytes(), CHALLENGE_5_KEY.as_bytes()),
            &hex_string_to_bytes(CHALLENGE_5_RESULT).unwrap()
        );
    }
}
