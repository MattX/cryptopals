use std::collections::HashMap;
use std::str;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
static FREQUENCIES: [f32; 26] = [
    0.08167,
    0.01492,
    0.02782,
    0.04253,
    0.12702,
    0.02228,
    0.02015,
    0.06094,
    0.06966,
    0.00153,
    0.00772,
    0.04025,
    0.02406,
    0.06749,
    0.07507,
    0.01929,
    0.00095,
    0.05987,
    0.06327,
    0.09056,
    0.02758,
    0.00978,
    0.0236,
    0.0015,
    0.01974,
    0.00074
];

pub fn evaluate(string: &str) -> f32 {
    string.chars().fold(0.0, |tot, c| {
        tot +
            (if c.is_ascii_alphanumeric() {
                0.0
            } else if c.is_ascii_whitespace() {
                0.1
            } else if c.is_ascii_punctuation() {
                0.2
            } else {
                1.0
            })
    })
}

pub fn evaluate_alt(string: &str) -> f32 {
    let counter_map = counter(&str::to_ascii_lowercase(string));
    let freq_map = counter_to_freq(&counter_map);

    let this_vector = ALPHABET.chars().map(|c| {
        freq_map.get(&c).unwrap_or(&0.0).clone()
    }).collect::<Vec<f32>>();

    this_vector.iter().zip(FREQUENCIES.iter()).map(|(c, r)| (c - r).powi(2)).sum::<f32>().sqrt()
}

fn counter_to_freq(counter: &HashMap<char, u64>) -> HashMap<char, f32> {
    let total = counter.values().fold(0, |x, y| x + y);
    let mut frequency_map = HashMap::new();

    counter.iter().for_each(|(k, v)| {
        frequency_map.insert(*k, *v as f32 / total as f32);
    });

    frequency_map
}

fn counter(string: &str) -> HashMap<char, u64> {
    let mut counter_map: HashMap<char, u64> = HashMap::new();

    string.chars().for_each(|c| {
        counter_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
    });

    counter_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_string() {
        assert_eq!(counter("adabcade"),
                   [('a', 3), ('b', 1), ('c', 1), ('d', 2), ('e', 1)].iter().cloned().collect::<HashMap<char, u64>>());
    }
}
