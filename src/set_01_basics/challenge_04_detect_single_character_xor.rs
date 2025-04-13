use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::challenge_03_single_byte_xor_cipher::{
    score_english_text, single_byte_xor_cipher, SingleByteXorDecryptionResult,
};

pub fn detect_single_character_xor(path: &str) -> SingleByteXorDecryptionResult {
    let p = Path::new(path);
    let file = File::open(p).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let ciphertext = line.unwrap();
            single_byte_xor_cipher(ciphertext.as_bytes())
        })
        .max_by(|result_x, result_y| {
            score_english_text(&result_x.plaintext)
                .total_cmp(&score_english_text(&result_y.plaintext))
        })
        .expect("Should always find a best key")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_single_character_xor() {
        let test_cases = vec![(
            String::from("resources/set_01_basics/4.txt"),
            SingleByteXorDecryptionResult {
                plaintext: b"Now that the party is jumping\n".to_vec(),
                key: b'5',
            },
        )];
        for (path, expected_result) in test_cases {
            assert_eq!(expected_result, detect_single_character_xor(&path));
        }
    }
}
