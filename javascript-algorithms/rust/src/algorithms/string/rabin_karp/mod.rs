use crate::algorithms::cryptography::polynomial_hash::PolynomialHash;

/// Where `word` first is in `text`, by Rabin and Karp's rolling hash; -1
/// where it isn't.
pub fn rabin_karp(text: &str, word: &str) -> isize {
    let hasher = PolynomialHash::new(None);

    // Calculate word hash that we will use for comparison with other substring hashes.
    let word_hash = hasher.hash(word);

    let text: Vec<char> = text.chars().collect();
    let word_length = word.chars().count();
    if word_length > text.len() {
        return -1;
    }

    let mut prev_frame: Option<String> = None;
    let mut current_frame_hash = None;

    // Go through all substring of the text that may match.
    for char_index in 0..=text.len() - word_length {
        let current_frame: String = text[char_index..char_index + word_length].iter().collect();

        // Calculate the hash of current substring.
        let frame_hash = match (current_frame_hash, &prev_frame) {
            (Some(hash), Some(prev_frame)) => hasher.roll(hash, prev_frame, &current_frame),
            _ => hasher.hash(&current_frame),
        };
        current_frame_hash = Some(frame_hash);

        // Compare the hash of current substring and seeking string.
        // In case if hashes match let's make sure that substrings are equal.
        // In case of hash collision the strings may not be equal.
        if word_hash == frame_hash && current_frame == word {
            return char_index as isize;
        }

        prev_frame = Some(current_frame);
    }

    -1
}

js::export_default!(rabin_karp);
