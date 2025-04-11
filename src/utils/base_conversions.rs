const HEX_RADIX: u8 = 16;

const BASE64_ENCODING_TABLE: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

pub fn hex_to_base64(hex: &[u8]) -> Vec<u8> {
    let bytes = hex_to_bytes(hex);
    bytes_to_base64(&bytes)
}

pub fn hex_to_bytes(hex: &[u8]) -> Vec<u8> {
    assert!(hex.len() % 2 == 0);

    hex.chunks(2)
        .map(|chunk| hex_pair_to_byte(chunk[0], chunk[1]))
        .collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .flat_map(|byte| {
            let (high, low) = byte_to_hex_pair(*byte);
            [high, low]
        })
        .collect()
}

pub fn bytes_to_base64(bytes: &[u8]) -> Vec<u8> {
    let mut base64 = Vec::new();

    for chunk in bytes.chunks(3) {
        let (octet1, octet2, octet3) = match chunk.len() {
            3 => (chunk[0], chunk[1], chunk[2]),
            2 => (chunk[0], chunk[1], 0),
            1 => (chunk[0], 0, 0),
            _ => unreachable!("Invalid chunk length"),
        };

        let sextet1 = octet1 >> 2;
        let sextet2 = ((octet1 & 0b11) << 4) | (octet2 >> 4);
        let sextet3 = ((octet2 & 0b1111) << 2) | (octet3 >> 6);
        let sextet4 = octet3 & 0b111111;

        base64.push(BASE64_ENCODING_TABLE[sextet1 as usize]);
        base64.push(BASE64_ENCODING_TABLE[sextet2 as usize]);

        // Handle "=" padding
        if chunk.len() >= 2 {
            base64.push(BASE64_ENCODING_TABLE[sextet3 as usize]);
        } else {
            base64.push(b'=');
        }

        if chunk.len() == 3 {
            base64.push(BASE64_ENCODING_TABLE[sextet4 as usize]);
        } else {
            base64.push(b'=');
        }
    }

    base64
}

pub fn hex_pair_to_byte(high: u8, low: u8) -> u8 {
    hex_digit_to_decimal(high) * HEX_RADIX + hex_digit_to_decimal(low)
}

pub fn hex_digit_to_decimal(digit: u8) -> u8 {
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'a'..=b'f' => digit - b'a' + 10,
        _ => unreachable!("Invalid hex digit ({})", digit),
    }
}

pub fn byte_to_hex_pair(decimal: u8) -> (u8, u8) {
    (
        decimal_to_hex_digit(decimal / HEX_RADIX),
        decimal_to_hex_digit(decimal & 0b1111),
    )
}

pub fn decimal_to_hex_digit(decimal: u8) -> u8 {
    match decimal {
        0..=9 => decimal + b'0',
        10..=15 => decimal + b'a' - 10,
        _ => unreachable!("Invalid decimal ({})", decimal),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let test_cases: Vec<(&[u8], Vec<u8>)> = vec![
            (b"0000", b"AAA=".to_vec()),
            (b"00", b"AA==".to_vec()),
            (b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_vec()),
            (b"3b736c646a66206f73646a20706d7073646d66706f736f706476666d73646f70766670", b"O3NsZGpmIG9zZGogcG1wc2RtZnBvc29wZHZmbXNkb3B2ZnA=".to_vec()),
            (b"3b736c646a66206f73646a6d20706d353876703973646d66706f736f706476666d73646f70766670", b"O3NsZGpmIG9zZGptIHBtNTh2cDlzZG1mcG9zb3BkdmZtc2RvcHZmcA==".to_vec())
            ];
        for (hex, expected_result) in test_cases {
            assert_eq!(expected_result, hex_to_base64(hex));
        }
    }
    #[test]
    fn test_bytes_to_base64() {
        let test_cases = vec![
            (vec![0, 0], b"AAA=".to_vec()),
            (vec![0], b"AA==".to_vec()),
            (vec![14, 15, 16], b"Dg8Q".to_vec()),
        ];
        for (bytes, expected_result) in test_cases {
            assert_eq!(expected_result, bytes_to_base64(&bytes));
        }
    }

    #[test]
    fn test_hex_to_bytes() {
        let test_cases = vec![(b"49276d00", vec![73, 39, 109, 0])];
        for (hex, expected_result) in test_cases {
            assert_eq!(expected_result, hex_to_bytes(hex));
        }
    }

    #[test]
    fn test_bytes_to_hex() {
        let test_cases = vec![(vec![73, 39, 109, 0], b"49276d00".to_vec())];
        for (bytes, expected_result) in test_cases {
            assert_eq!(expected_result, bytes_to_hex(&bytes));
        }
    }

    #[test]
    fn test_hex_pair_to_byte() {
        let test_cases = vec![(b'4', b'f', 79), (b'a', b'a', 170), (b'0', b'0', 0)];
        for (high, low, expected_result) in test_cases {
            assert_eq!(expected_result, hex_pair_to_byte(high, low));
        }
    }

    #[test]
    fn test_byte_to_hex_pair() {
        let test_cases = vec![(79, (b'4', b'f')), (170, (b'a', b'a')), (0, (b'0', b'0'))];
        for (byte, expected_result) in test_cases {
            assert_eq!(expected_result, byte_to_hex_pair(byte));
        }
    }

    #[test]
    fn test_hex_digit_to_decimal() {
        let test_cases = vec![(b'0', 0), (b'5', 5), (b'a', 10), (b'e', 14)];
        for (digit, expected_result) in test_cases {
            assert_eq!(expected_result, hex_digit_to_decimal(digit));
        }
    }
    #[test]
    fn test_decimal_to_hex_digit() {
        let test_cases = vec![(0, b'0'), (5, b'5'), (10, b'a'), (14, b'e')];
        for (digit, expected_result) in test_cases {
            assert_eq!(expected_result, decimal_to_hex_digit(digit));
        }
    }
}
