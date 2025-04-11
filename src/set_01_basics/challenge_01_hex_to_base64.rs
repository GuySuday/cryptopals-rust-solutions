use crate::utils;

pub fn hex_to_base64(hex: &[u8]) -> Vec<u8> {
    utils::base_conversions::hex_to_base64(hex)
}
