// The string separator that is being used for "word" and "text" concatenation.
const SEPARATOR: &str = "$";

/// For each of `z_string`'s positions, how long a prefix of it starts there.
fn build_z_array(z_string: &[u8]) -> Vec<usize> {
    // Initiate zArray and fill it with zeros.
    let mut z_array = vec![0; z_string.len()];

    // Z box boundaries.
    let mut z_box_left_index = 0;
    let mut z_box_right_index = 0;

    // Go through all characters of the zString.
    for char_index in 1..z_string.len() {
        if char_index > z_box_right_index {
            // We're OUTSIDE of Z box, so build a new one.
            z_box_left_index = char_index;
            z_box_right_index = char_index;

            while z_box_right_index < z_string.len()
                && z_string[z_box_right_index - z_box_left_index] == z_string[z_box_right_index]
            {
                z_box_right_index += 1;
            }

            z_array[char_index] = z_box_right_index - z_box_left_index;
            z_box_right_index -= 1;
        } else {
            // We're INSIDE of Z box: how far from its start this character is.
            let z_box_shift = char_index - z_box_left_index;

            if z_array[z_box_shift] < (z_box_right_index - char_index) + 1 {
                z_array[char_index] = z_array[z_box_shift];
            } else {
                z_box_left_index = char_index;

                while z_box_right_index < z_string.len()
                    && z_string[z_box_right_index - z_box_left_index] == z_string[z_box_right_index]
                {
                    z_box_right_index += 1;
                }

                z_array[char_index] = z_box_right_index - z_box_left_index;
                z_box_right_index -= 1;
            }
        }
    }

    z_array
}

/// Each position `word` is at in `text`, by the Z algorithm.
pub fn z_algorithm(text: &str, word: &str) -> Vec<usize> {
    // The list of word's positions in text.
    let mut word_positions = Vec::new();

    // Concatenate word and string. Word will be a prefix to a string.
    let z_string = format!("{word}{SEPARATOR}{text}");

    // Generate Z-array for concatenated string.
    let z_array = build_z_array(z_string.as_bytes());

    // Based on Z-array properties each cell will tell us the length of the match between
    // the string prefix and current sub-text. Thus we're may find all positions in zArray
    // with the number that equals to the length of the word (zString prefix) and based on
    // that positions we'll be able to calculate word positions in text.
    for (char_index, &length) in z_array.iter().enumerate().skip(1) {
        if length == word.len() {
            let word_position = char_index - word.len() - SEPARATOR.len();
            word_positions.push(word_position);
        }
    }

    word_positions
}

js::export_default!(z_algorithm);
