use std::collections::HashMap;

const ENGLISH_ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Generates a cipher map out of the alphabet: each character the one
/// `shift` after it, round the alphabet's end.
fn get_cipher_map(alphabet: &[char], shift: i32) -> HashMap<char, char> {
    alphabet
        .iter()
        .enumerate()
        .map(|(char_index, &current_char)| {
            let length = alphabet.len() as i32;
            let mut encrypted_char_index = (char_index as i32 + shift) % length;
            // Support negative shifts for creating a map for decryption
            // (see caesarCipherDecrypt() function below).
            if encrypted_char_index < 0 {
                encrypted_char_index += length;
            }
            (current_char, alphabet[encrypted_char_index as usize])
        })
        .collect()
}

/// `str` lowercased, each of `alphabet`'s characters `shift` along it.
pub fn caesar_cipher_encrypt(str: &str, shift: i32, alphabet: Option<&[char]>) -> String {
    // Create a cipher map:
    let cipher_map = get_cipher_map(alphabet.unwrap_or(&ENGLISH_ALPHABET), shift);
    str.to_lowercase()
        .chars()
        .map(|char| cipher_map.get(&char).copied().unwrap_or(char))
        .collect()
}

/// `str` lowercased, each of `alphabet`'s characters `shift` back along it.
pub fn caesar_cipher_decrypt(str: &str, shift: i32, alphabet: Option<&[char]>) -> String {
    // Create a cipher map:
    let cipher_map = get_cipher_map(alphabet.unwrap_or(&ENGLISH_ALPHABET), -shift);
    str.to_lowercase()
        .chars()
        .map(|char| cipher_map.get(&char).copied().unwrap_or(char))
        .collect()
}
