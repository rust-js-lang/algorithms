use crate::algorithms::math::matrix::{self as mtrx, Matrix};

const ALPHABET_CODE_SHIFT: u32 = 'A' as u32;
const ENGLISH_ALPHABET_SIZE: u32 = 26;

/// Generates key matrix from given keyString.
fn generate_key_matrix(key_string: &str) -> Matrix {
    let matrix_size = (key_string.len() as f64).sqrt();
    if matrix_size.fract() != 0.0 {
        panic!("Invalid key string length. The square root of the key string must be an integer");
    }
    let mut key_string_index = key_string.chars();
    mtrx::generate(&[matrix_size as usize, matrix_size as usize], |_| {
        // A → 0, B → 1, ..., a → 32, b → 33, ...
        let char_code_shifted = key_string_index.next().map_or(0, u32::from) % ALPHABET_CODE_SHIFT;
        f64::from(char_code_shifted)
    })
}

/// Generates a message vector from a given message.
fn generate_message_vector(message: &str) -> Matrix {
    let codes: Vec<u32> = message.chars().map(u32::from).collect();
    mtrx::generate(&[message.len(), 1], |cell_indices| {
        let row_index = cell_indices[0];
        f64::from(codes[row_index] % ALPHABET_CODE_SHIFT)
    })
}

/// Encodes the given message using Hill Cipher.
pub fn hill_cipher_encrypt(message: &str, key_string: &str) -> String {
    // The message and key string can only contain letters.
    let only_letters =
        |text: &str| !text.is_empty() && text.chars().all(|char| char.is_ascii_alphabetic());
    if !only_letters(message) || !only_letters(key_string) {
        panic!("The message and key string can only contain letters");
    }

    let key_matrix = generate_key_matrix(key_string);
    let message_vector = generate_message_vector(message);

    // keyString.length must equal to square of message length
    if mtrx::shape(&key_matrix)[0] != message.len() {
        panic!("Invalid key string length. The key length must be a square of message length");
    }

    let cipher_vector = mtrx::dot(&key_matrix, &message_vector);
    let mut cipher_string = String::new();
    for row in 0..message.len() {
        let item = mtrx::get_cell_at_index(&cipher_vector, &[row, 0]) as u32;
        cipher_string.push(
            char::from_u32((item % ENGLISH_ALPHABET_SIZE) + ALPHABET_CODE_SHIFT).unwrap_or('A'),
        );
    }

    cipher_string
}

/// Decodes a message encoded with Hill Cipher: not yet.
pub fn hill_cipher_decrypt() -> String {
    panic!("This method is not implemented yet");
}
