use openssl::symm::{decrypt, Cipher};

pub fn decrypt_aes_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Box<[u8]>, String> {
  let cipher = Cipher::aes_128_ecb();

  decrypt(cipher, key, None, ciphertext)
      .map(|x| x.into_boxed_slice())
      .map_err(|e| e.to_string())
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
}
