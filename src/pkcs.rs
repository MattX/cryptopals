pub fn pkcs7(block_size: u8, data_im: Vec<u8>) -> Vec<u8> {
  let mut data = data_im;

  // This cast is guaranteed not to overflow since block_size is a u8.
  let last_block_size = (data.len() % block_size as usize) as u8;
  if last_block_size == 0 {
    return data;
  }

  let to_add = block_size - last_block_size;
  for i in 0..to_add {
    data.push(to_add);
  }
  data
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn challenge_9() {
    let data = "YELLOW SUBMARINE";
    let data_vec = data.as_bytes().to_vec();

    assert_eq!(pkcs7(20, data_vec), b"YELLOW SUBMARINE\x04\x04\x04\x04");
  }
}
