//! Functions for stuff

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering::*;
use std::f32;
use num::Integer;
use xor;
use evaluate;
use hex_io;
use hamming;

pub fn multi_xor(filename: &str) -> String {
  let file = File::open(filename)
      .unwrap_or_else(|e| panic!("File {} not found", filename));
  let best = BufReader::new(file).lines()
      .map(|l| single_xor(
        &hex_io::hex_string_to_bytes(&l.unwrap()).unwrap()
      ))
      .min_by(|(v1, k1, s1), (v2, k2, s2)| s1.partial_cmp(s2).unwrap_or(Equal))
      .map(|(v, k, s)| v)
      .unwrap();

  best
}

pub fn find_best_xor(encrypted: &[u8]) -> String {
  let (val, key, score) = single_xor(encrypted);
  val
}

pub fn single_xor(encrypted: &[u8]) -> (String, u8, f32) {
  let mut best_value: String = String::new();
  let mut best_key: u8 = 0;
  let mut best_score: f32 = f32::INFINITY;

  for i in 0..255 as u8 {
    let key = vec![i; encrypted.len()];
    let result = xor::xor(encrypted, &key);
    let str_result: String = result.iter().map(|c| *c as char).collect();
    let score = evaluate::evaluate(&str_result);
    if score < best_score {
      best_score = score;
      best_key = i;
      best_value = str_result;
    }
  }

  (best_value, best_key, best_score)
}

pub fn find_key_size(buffer: &[u8]) -> usize {
  if buffer.len() < 80 {
    panic!("Buffer too short to determine key size");
  }

  let score = |i: usize| (hamming::bit_hamming(&buffer[0..i], &buffer[i..2*i]) as f64) / (i as f64);

  (2_usize..40)
      .min_by(|&i, &j| score(i).partial_cmp(&score(j)).unwrap_or(Equal))
      .unwrap()
}

pub fn transpose_blocks(buffer: &[u8], key_length: usize) -> Vec<Vec<u8>> {
  let n_full_blocks= (buffer.len() + key_length - 1) / key_length;
  let mut transposed: Vec<Vec<u8>> = Vec::with_capacity(key_length);

  for i in 0..key_length {
    transposed.push(Vec::with_capacity(n_full_blocks));
  }

  for i in 0..n_full_blocks {
    for j in 0..key_length {
      let source_index = j + i * key_length;
      if source_index < buffer.len() {
        transposed[j].push(buffer[source_index]);
      }
    }
  }

  transposed
}

pub fn best_rep(buffer: &[u8]) -> Box<[u8]> {
  let key_size = find_key_size(buffer);
  let transposed = transpose_blocks(buffer, key_size);

  let strs :Vec<String> = transposed.iter().map(|col| find_best_xor(&col)).collect();
  let max_len = strs.iter().map(|s| s.len()).max().unwrap();

  for i in 0..strs.len() {
  }

  vec![].into_boxed_slice()
}

#[cfg(test)]
mod test {
  use super::*;
  use hex_io::*;

  #[test]
  fn test_transpose() {
    assert_eq!(
      transpose_blocks(&[0, 1, 2, 3, 4], 2),
      vec![vec![0, 2, 4], vec![1, 3]]
    )
  }

  const CHALLENGE_3_DATA: &'static str =
    "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

  #[test]
  fn challenge_3() {
    let (res, key, score) = single_xor(&hex_string_to_bytes(CHALLENGE_3_DATA).unwrap());
    assert_eq!(key, 88);
    assert_eq!(&res, "Cooking MC's like a pound of bacon");
  }
}
