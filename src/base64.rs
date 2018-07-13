static CHARACTERS: [u8; 65] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
    88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
    43, 47, 61
];
static FILLER_VAL: u8 = 0x40;

fn to_base64(bytes: &[u8]) -> Box<[u8]> {
    let mut result: Box<[u8]> = to_base64_raw(bytes);

    for i in 0..result.len() {
        result[i] = CHARACTERS[result[i] as usize];
    }

    result
}

fn to_base64_raw(bytes: &[u8]) -> Box<[u8]> {
    let num_blocks = (bytes.len() + 2) / 3;
    let output_len = num_blocks * 4;
    let mut output: Box<[u8]> = vec![0; output_len].into_boxed_slice();

    for i in 0..num_blocks - 1 {
        block_to_base64(&bytes[i * 3..(i + 1) * 3], &mut output[i * 4..(i + 1) * 4]);
    }
    let last_block = num_blocks - 1;
    block_to_base64(&bytes[last_block * 3..bytes.len()],
                    &mut output[last_block * 4..(last_block + 1) * 4]);

    output
}

// Result must be zeroed out.
fn block_to_base64(block: &[u8], result: &mut [u8]) -> () {
    let len = block.len();
    if len < 1 || len > 3 {
        panic!("Got a block of {} bytes", len)
    }

    if result.len() != 4 {
        panic!("Got a result block of {} bytes", result.len())
    }

    result[0] = (block[0] & 0xfc) >> 2;
    result[1] = (block[0] & 0x03) << 4;

    if len == 1 {
        result[2] = FILLER_VAL;
        result[3] = FILLER_VAL;
    } else {
        result[1] |= (block[1] & 0xf0) >> 4;
        result[2] = (block[1] & 0x0f) << 2;

        if len == 2 {
            result[3] = FILLER_VAL;
        } else {
            result[2] |= (block[2] & 0xc0) >> 6;
            result[3] = block[2] & 0x3f;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn convert_block_full() {
        let mut result: [u8; 4] = [0; 4];
        block_to_base64(&[77, 97, 110], &mut result);
        assert_eq!(result, [19, 22, 5, 46]);
    }

    #[test]
    fn convert_block_partial() {
        let mut result: [u8; 4] = [0; 4];
        block_to_base64(&[77, 97], &mut result);
        assert_eq!(result, [19, 22, 4, 64]);

        result = [0; 4];
        block_to_base64(&[77], &mut result);
        assert_eq!(result, [19, 16, 64, 64]);
    }

    #[test]
    fn to_base64_raw_block() {
        assert_eq!(&to_base64_raw(&[77, 97, 110])[..], [19, 22, 5, 46]);
    }

    #[test]
    fn to_base64_full() {
        let test_str: [u8; 20] = [97, 110, 121, 32, 99, 97, 114, 110, 97, 108, 32, 112, 108,
            101, 97, 115, 117, 114, 101, 46];
        assert_eq!(str::from_utf8(&to_base64(&test_str)), Ok("YW55IGNhcm5hbCBwbGVhc3VyZS4="));
    }
}