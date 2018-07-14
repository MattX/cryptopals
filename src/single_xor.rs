use std::str;
use std::f32;
use xor;
use evaluate;

pub fn find_best_xor(encrypted: &[u8]) -> String {
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

    best_value
}
