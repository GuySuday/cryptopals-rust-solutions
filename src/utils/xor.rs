use crate::utils;

pub fn xor_hex(hex1: &[u8], hex2: &[u8]) -> Vec<u8> {
    let bytes1 = utils::base_conversions::hex_to_bytes(hex1);
    let bytes2 = utils::base_conversions::hex_to_bytes(hex2);

    let xored_bytes = utils::xor::xor_bytes(&bytes1, &bytes2);

    utils::base_conversions::bytes_to_hex(&xored_bytes)
}

pub fn xor_bytes(buffer1: &[u8], buffer2: &[u8]) -> Vec<u8> {
    buffer1
        .iter()
        .zip(buffer2.iter())
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_hex() {
        let test_cases = vec![(
            b"1c0111001f010100061a024b53535009181c",
            b"686974207468652062756c6c277320657965",
            b"746865206b696420646f6e277420706c6179".to_vec(),
        )];
        for (buffer1, buffer2, expected_result) in test_cases {
            assert_eq!(expected_result, xor_hex(buffer1, buffer2));
        }
    }

    #[test]
    fn test_xor_bytes() {
        let test_cases = vec![(
            vec![5, 6, 7, 8, 100],
            vec![100, 100, 100, 100, 100],
            vec![97, 98, 99, 108, 0],
        )];
        for (buffer1, buffer2, expected_result) in test_cases {
            assert_eq!(expected_result, xor_bytes(&buffer1, &buffer2));
        }
    }
}
