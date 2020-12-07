use std::ascii::escape_default;

pub fn bytes_to_string(s: &[u8]) -> String {
    s.iter()
        .map(|c| escape_default(*c).map(|sc| sc as char).collect::<String>())
        .collect::<Vec<String>>()
        .concat()
}
