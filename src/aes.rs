use openssl::symm::{decrypt, Cipher, Crypter, Mode};
use std::collections::HashMap;
use xor::xor;

const AES_BLOCK_SIZE: usize = 16;

pub fn decrypt_aes_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Box<[u8]>, String> {
    if ciphertext.len() % 16 != 0 {
        panic!(
            "decrypt_aes_ecb called with invalid text length {}.",
            ciphertext.len()
        );
    }
    match key.len() {
        16 | 24 | 32 => (),
        _ => panic!(
            "decrypt_aes_ecb called with invalid key length {}.",
            key.len()
        ),
    }

    let cipher = Cipher::aes_128_ecb();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
    decrypter.pad(false);

    let mut out = vec![0 as u8; ciphertext.len() + AES_BLOCK_SIZE];
    let size_written = decrypter
        .update(ciphertext, &mut out)
        .map_err(|e| e.to_string())?;

    let mut decrypted = Vec::new();
    decrypted.extend_from_slice(&out[0..size_written]);

    let final_size = decrypter.finalize(&mut out).map_err(|e| e.to_string())?;
    decrypted.extend_from_slice(&out[0..final_size]);

    Ok(decrypted.into_boxed_slice())
}

pub fn decrypt_aes_cbc(
    ciphertext: &[u8],
    key: &[u8],
    iv: [u8; AES_BLOCK_SIZE],
) -> Result<Box<[u8]>, String> {
    let mut previous_ciphertext: &[u8] = &iv;
    let mut result = Vec::new();

    for chunk in ciphertext.chunks(AES_BLOCK_SIZE) {
        let decrypted = decrypt_aes_ecb(chunk, key)?;
        result.extend_from_slice(&xor(&decrypted, previous_ciphertext));
        previous_ciphertext = chunk;
    }

    Ok(result.into_boxed_slice())
}

fn score_duplicates(data: &[u8]) -> u64 {
    let mut counts = HashMap::new();

    for chunk in data.chunks(AES_BLOCK_SIZE) {
        let stat = counts
            .entry(chunk)
            .and_modify(|x| *x += 1)
            .or_insert(0 as u64);
    }

    counts.values().map(|x| x.pow(2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_io::*;
    use std::str::from_utf8;

    #[test]
    fn challenge_7() {
        let data = load_b64_from_file("data/7.txt").unwrap();
        let key = b"YELLOW SUBMARINE";
        let reference = "I'm back and I'm ringin' the bell";

        let decrypted = decrypt_aes_ecb(&data, key).unwrap();
        let result = from_utf8(&decrypted).unwrap();
        assert_eq!(&result[0..reference.len()], reference);
    }

    #[test]
    fn challenge_8() {
        let possibilities = read_hex_file("data/8.txt").unwrap();

        let scored = possibilities
            .iter()
            .map(|p| (score_duplicates(&p), p))
            .max_by_key(|(sp, p)| *sp)
            .unwrap()
            .1;

        let reference: Vec<u8> = vec![216, 128, 97, 151, 64, 168, 161];
        assert_eq!(scored[0..reference.len()], reference[..]);
    }

    #[test]
    fn challenge_10() {
        use util::bytes_to_string;

        let data = load_b64_from_file("data/10.txt").unwrap();
        let reference = "I\\'m back and I\\'m ringin\\' the bell";

        let key = b"YELLOW SUBMARINE";
        let iv = [0 as u8; AES_BLOCK_SIZE];

        let decrypted = decrypt_aes_cbc(&data, key, iv).unwrap();
        let result = bytes_to_string(&decrypted);
        assert_eq!(&result[0..reference.len()], reference);
    }
}
