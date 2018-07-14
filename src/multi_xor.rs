use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering::*;
use single_xor;
use hex_io;

pub fn multi_xor(filename: &str) -> String {
    let file = File::open(filename).unwrap_or_else(|e|panic!("File not found"));
    let best = BufReader::new(file).lines()
        .map(|l| single_xor::single_xor(
            &hex_io::hex_string_to_bytes(&l.unwrap()).unwrap()
        ))
        .min_by(|(v1, s1), (v2, s2)| s1.partial_cmp(s2).unwrap_or(Equal))
        .map(|(v, s)| v)
        .unwrap();

    best
}
