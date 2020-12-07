//! Functions for stuff

use std::cmp::Ordering::*;
use std::f32;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::from_utf8_unchecked;

use evaluate;
use hamming::*;
use xor;

/// Finds the key for a cipher text that was encrypted with a one-byte key.
pub fn single_xor(encrypted: &[u8]) -> (String, u8, f32) {
    let mut best_value: String = String::new();
    let mut best_key: u8 = 0;
    let mut best_score: f32 = f32::INFINITY;

    for i in 0..255 as u8 {
        let key = vec![i; encrypted.len()];
        let result = xor::xor(encrypted, &key);
        let score = evaluate::evaluate(&result);
        let mut str_result = String::new();
        unsafe {
            str_result = from_utf8_unchecked(&result).to_string();
        }
        if score < best_score {
            best_score = score;
            best_key = i;
            best_value = str_result;
        }
    }

    (best_value, best_key, best_score)
}

pub fn best_repeating_xor_keys(buffer: &[u8]) -> Vec<Box<[u8]>> {
    let key_sizes = guess_key_size(buffer);
    let mut keys: Vec<Box<[u8]>> = Vec::new();

    for key_size in key_sizes.iter() {
        let transposed = transpose_blocks(buffer, *key_size);
        let best_keys = transposed
            .iter()
            .map(|col| find_best_single_key(&col))
            .collect::<Vec<u8>>()
            .into_boxed_slice();
        keys.push(best_keys);
    }

    keys
}

/// Finds the most likely key sizes in a cipher text (between 2 and 40). Returns the 3 most likely
/// values.
///
/// Compares the hamming distance between the first few blocks for a range of possibilities.
fn guess_key_size(buffer: &[u8]) -> Box<[usize]> {
    const N_BLOCKS: usize = 4;
    const MAX_KEY_SIZE: usize = 40;

    if buffer.len() < MAX_KEY_SIZE * N_BLOCKS {
        panic!("Buffer too short to determine key size");
    }

    let score = |key_size: usize| {
        let mut total_distance = 0;
        let first_block = &buffer[0..key_size];
        for i in 1..N_BLOCKS {
            let second_block = &buffer[i * key_size..(i + 1) * key_size];
            total_distance += bit_hamming(first_block, second_block);
        }
        total_distance as f64 / (key_size as f64)
    };

    let mut res = (2_usize..MAX_KEY_SIZE)
        .map(|i| (i, score(i)))
        .collect::<Vec<(usize, f64)>>();
    res.sort_by(|(i, si), (j, sj)| si.partial_cmp(sj).unwrap_or(Equal));
    res.iter()
        .map(|(i, si)| *i)
        .take(3)
        .collect::<Vec<usize>>()
        .into_boxed_slice()
}

/// Helper function to return the best single key
fn find_best_single_key(encrypted: &[u8]) -> u8 {
    let (val, key, score) = single_xor(encrypted);
    key
}

fn transpose_blocks(buffer: &[u8], key_length: usize) -> Vec<Vec<u8>> {
    let mut transposed: Vec<Vec<u8>> = Vec::with_capacity(key_length);

    for i in 0..key_length {
        transposed.push(Vec::new());
    }

    for (i, c) in buffer.iter().enumerate() {
        let block = i % key_length;
        transposed[block].push(*c);
    }

    transposed
}

#[cfg(test)]
mod test {
    use super::*;
    use base64;
    use hex_io::*;
    use std::str::from_utf8;

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

    #[test]
    fn challenge_4() {
        let lines = read_hex_file("data/4.txt").unwrap();
        let (best_result, best_key, best_score) = lines
            .into_iter()
            .map(|l| single_xor(&l))
            .min_by(|(v1, k1, s1), (v2, k2, s2)| s1.partial_cmp(s2).unwrap_or(Equal))
            .unwrap();
        assert_eq!(&best_result, "Now that the party is jumping\n");
    }

    #[test]
    fn challenge_6_2() {
        let test_data = load_b64_from_file("data/6.txt").unwrap();
        let best_keys = best_repeating_xor_keys(&test_data);
        let mut decoded: Vec<Box<[u8]>> = Vec::new();

        for key in best_keys {
            decoded.push(xor::rep_key_xor(&test_data, &key));
        }

        let best = decoded
            .iter()
            .map(|d| (d, evaluate::evaluate(&d)))
            .min_by(|(a, sa), (b, sb)| sa.partial_cmp(sb).unwrap_or(Equal))
            .unwrap();

        let expected_result = "I\'m back and I\'m ringin\' the bell";
        assert_eq!(
            &from_utf8(best.0).unwrap()[0..expected_result.len()],
            expected_result
        );
    }
}
