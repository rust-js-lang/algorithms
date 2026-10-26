/// For each of `word`'s prefixes, the length of its longest proper prefix
/// that's also its suffix.
fn build_pattern_table(word: &[u8]) -> Vec<usize> {
    let mut pattern_table = vec![0; word.len().max(1)];
    let mut prefix_index = 0;
    let mut suffix_index = 1;

    while suffix_index < word.len() {
        if word[prefix_index] == word[suffix_index] {
            pattern_table[suffix_index] = prefix_index + 1;
            suffix_index += 1;
            prefix_index += 1;
        } else if prefix_index == 0 {
            pattern_table[suffix_index] = 0;
            suffix_index += 1;
        } else {
            prefix_index = pattern_table[prefix_index - 1];
        }
    }

    pattern_table
}

/// Where `word` first is in `text`, by Knuth, Morris and Pratt's search;
/// -1 where it isn't.
pub fn knuth_morris_pratt(text: &str, word: &str) -> isize {
    let (text, word) = (text.as_bytes(), word.as_bytes());
    if word.is_empty() {
        return 0;
    }

    let mut text_index = 0;
    let mut word_index = 0;

    let pattern_table = build_pattern_table(word);

    while text_index < text.len() {
        if text[text_index] == word[word_index] {
            // We've found a match.
            if word_index == word.len() - 1 {
                return (text_index + 1 - word.len()) as isize;
            }
            word_index += 1;
            text_index += 1;
        } else if word_index > 0 {
            word_index = pattern_table[word_index - 1];
        } else {
            text_index += 1;
        }
    }

    -1
}

js::export_default!(knuth_morris_pratt);
