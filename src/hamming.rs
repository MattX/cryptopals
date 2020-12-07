//! Computes hamming distances

pub fn bit_hamming(a: &[u8], b: &[u8]) -> u64 {
    if a.len() != b.len() {
        panic!("Mismatched lengths ({} vs {})", a.len(), b.len())
    }

    a.iter()
        .zip(b)
        .map(|(v_a, v_b)| count_bits(v_a ^ v_b))
        .sum()
}

#[inline]
fn count_bits(a: u8) -> u64 {
    let mut count: u64 = 0;
    let mut val: u8 = a;
    while val != 0 {
        count += 1;
        val &= val - 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_bits() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(0b1111_1111), 8);
        assert_eq!(count_bits(0b1000_0001), 2);
    }

    #[test]
    pub fn challenge_6_2() {
        assert_eq!(
            bit_hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
            37
        )
    }
}
