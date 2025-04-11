use crate::utils::{self};

pub fn fixed_xor(hex1: &[u8], hex2: &[u8]) -> Vec<u8> {
    utils::xor::xor_hex(hex1, hex2)
}
