use std::collections::HashMap;
use openssl::symm::{decrypt, Cipher};

pub fn decrypt_aes_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Box<[u8]>, String> {
  let cipher = Cipher::aes_128_ecb();

  decrypt(cipher, key, None, ciphertext)
      .map(|x| x.into_boxed_slice())
      .map_err(|e| e.to_string())
}

pub fn score_duplicates(data: &[u8]) -> u64 {
  let mut counts = HashMap::new();

  for chunk in data.chunks(16) {
    let stat = counts.entry(chunk)
        .and_modify(|x| *x += 1)
        .or_insert(0 as u64);
  }

  counts.values().map(|x| x.pow(2)).sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::from_utf8;
  use hex_io::*;

  #[test]
  fn challenge_7() {
    let data = load_b64_from_file("data/7.txt");
    let key = b"YELLOW SUBMARINE";
    let reference = "I\'m back and I\'m ringin\' the bell";

    let decrypted = decrypt_aes_ecb(&data, key).unwrap();
    let result = from_utf8(&decrypted).unwrap();
    assert_eq!(&result[0..reference.len()], reference);
  }

  #[test]
  fn challenge_8() {
    let possibilities = read_hex_file("data/8.txt").unwrap();

    let scored = possibilities.iter()
        .map(|p| (score_duplicates(&p), p))
        .max_by_key(|(sp, p)| *sp)
        .unwrap()
        .1;

    let reference: Vec<u8> = vec![216, 128, 97, 151, 64, 168, 161];
    assert_eq!(scored[0..reference.len()], reference[..]);
  }
}
