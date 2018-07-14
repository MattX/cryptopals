use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering::*;
use std::f32;
use xor;
use evaluate;
use hex_io;

pub fn multi_xor(filename: &str) -> String {
    let file = File::open(filename).unwrap_or_else(|e|panic!("File not found"));
    let best = BufReader::new(file).lines()
        .map(|l| single_xor(
            &hex_io::hex_string_to_bytes(&l.unwrap()).unwrap()
        ))
        .min_by(|(v1, s1), (v2, s2)| s1.partial_cmp(s2).unwrap_or(Equal))
        .map(|(v, s)| v)
        .unwrap();

    best
}

pub fn find_best_xor(encrypted: &[u8]) -> String {
    let (val, score) = single_xor(encrypted);
    val
}

pub fn single_xor(encrypted: &[u8]) -> (String, f32) {
    let mut best_value: String = String::new();
    let mut best_score: f32 = f32::INFINITY;

    for i in 0..255 as u8 {
        let key = vec![i; encrypted.len()];
        let result = xor::xor(encrypted, &key);
        let str_result: String = result.iter().map(|c| *c as char).collect();
        let score = evaluate::evaluate(&str_result);
        //println!("{} -> {}", str_result, score);
        if score < best_score {
            best_score = score;
            best_value = str_result;
        }
    }

    (best_value, best_score)
}