use crate::utils::{base_conversions, xor};

use phf::{phf_map, Map};

// Source:
// Relative Frequencies of Letters in General English Plain text
// From Cryptographical Mathematics, by Robert Edward Lewand:
// https://cs.wellesley.edu/~fturbak/codman/letterfreq.html
const ENGLISH_LETTERS_FREQUENCY: Map<u8, f32> = phf_map!(
    b'a' => 8.167,
    b'b' => 1.492,
    b'c' => 2.782,
    b'd' => 4.253,
    b'e' => 12.702,
    b'f' => 2.228,
    b'g' => 2.015,
    b'h' => 6.094,
    b'i' => 6.966,
    b'j' => 0.153,
    b'k' => 0.772,
    b'l' => 4.025,
    b'm' => 2.406,
    b'n' => 6.749,
    b'o' => 7.507,
    b'p' => 1.929,
    b'q' => 0.095,
    b'r' => 5.987,
    b's' => 6.327,
    b't' => 9.056,
    b'u' => 2.758,
    b'v' => 0.978,
    b'w' => 2.360,
    b'x' => 0.150,
    b'y' => 1.974,
    b'z' => 0.074,
);

#[derive(PartialEq, Debug)]
pub struct SingleByteXorDecryptionResult {
    pub plaintext: Vec<u8>,
    pub key: u8,
}

pub fn single_byte_xor_cipher(ciphertext: &[u8]) -> SingleByteXorDecryptionResult {
    detect_single_byte_xor_key(ciphertext)
}

pub fn score_english_text(text: &[u8]) -> f64 {
    if text.iter().any(|c| !is_printable_ascii(*c)) {
        return 0.0;
    }

    text.iter()
        .map(|byte| *ENGLISH_LETTERS_FREQUENCY.get(byte).unwrap_or(&0f32) as f64)
        .sum()
}

pub fn is_printable_ascii(ascii: u8) -> bool {
    matches!(ascii, 32..=126 | b'\n' | b'\r' | b'\t')
}

pub fn detect_single_byte_xor_key(ciphertext: &[u8]) -> SingleByteXorDecryptionResult {
    let length = ciphertext.len();
    let text_bytes = base_conversions::hex_to_bytes(ciphertext);

    (0..=255)
        .map(|byte| {
            let key: Vec<u8> = std::iter::repeat_n(byte, length).collect();
            let plaintext = xor::xor_bytes(&text_bytes, &key);
            let score = score_english_text(&plaintext);
            (
                score,
                SingleByteXorDecryptionResult {
                    plaintext: plaintext,
                    key: byte,
                },
            )
        })
        .max_by(|(score_x, _), (score_y, _)| score_x.total_cmp(score_y))
        .map(|(_, single_byte_xor_decryption_result)| single_byte_xor_decryption_result)
        .expect("Should always find a best key")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_single_byte_xor_key() {
        let test_cases: Vec<(&[u8], SingleByteXorDecryptionResult)> = vec![
            (
                b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
                SingleByteXorDecryptionResult {
                    plaintext: b"Cooking MC's like a pound of bacon".to_vec(),
                    key: b'X',
                },
            ),
            (
                b"7d5a5c5b0f465c0f5c4e494a0f4e414b0f494e5c5b0e",
                SingleByteXorDecryptionResult {
                    plaintext: b"Rust is safe and fast!".to_vec(),
                    key: b'/',
                },
            ),
            (
                b"7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f",
                SingleByteXorDecryptionResult {
                    plaintext: b"Now that the party is jumping\n".to_vec(),
                    key: b'5',
                },
            ),
        ];
        for (ciphertext, expected_result) in test_cases {
            assert_eq!(expected_result, detect_single_byte_xor_key(ciphertext));
        }
    }
}
