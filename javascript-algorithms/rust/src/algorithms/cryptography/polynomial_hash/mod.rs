const DEFAULT_BASE: f64 = 37.0;
const DEFAULT_MODULUS: f64 = 101.0;

/// A polynomial hash's base and modulus, each its default where it's left
/// out.
#[derive(Default)]
pub struct Options {
    pub base: Option<f64>,
    pub modulus: Option<f64>,
}

/// A polynomial hash of words, rolled a character at a time.
#[cfg_attr(rust_js, rust_js::class)]
pub struct PolynomialHash {
    pub base: f64,
    pub modulus: f64,
}

unsafe impl js::Defined for PolynomialHash {}

#[cfg_attr(rust_js, rust_js::class)]
impl PolynomialHash {
    /// @param {number} [base] - Base number that is used to create the polynomial.
    /// @param {number} [modulus] - Modulus number that keeps the hash from overflowing.
    pub fn new(options: Option<Options>) -> Self {
        let options = options.unwrap_or_default();
        PolynomialHash {
            base: options.base.unwrap_or(DEFAULT_BASE),
            modulus: options.modulus.unwrap_or(DEFAULT_MODULUS),
        }
    }

    /// Function that creates hash representation of the word.
    ///
    /// Time complexity: O(word.length).
    pub fn hash(&self, word: &str) -> f64 {
        let char_codes: Vec<f64> = word.chars().map(|char| self.char_to_number(char)).collect();

        let mut hash = 0.0;
        for char_code in char_codes {
            hash *= self.base;
            hash += char_code;
            hash %= self.modulus;
        }

        hash
    }

    /// Function that creates hash representation of the word
    /// based on previous word (shifted by one character left) hash value.
    ///
    /// Recalculates the hash representation of a word so that it isn't
    /// necessary to traverse the whole word again.
    ///
    /// Time complexity: O(1).
    pub fn roll(&self, prev_hash: f64, prev_word: &str, new_word: &str) -> f64 {
        let mut hash = prev_hash;

        let prev_value = prev_word
            .chars()
            .next()
            .map_or(0.0, |char| self.char_to_number(char));
        let new_value = new_word
            .chars()
            .last()
            .map_or(0.0, |char| self.char_to_number(char));

        let mut prev_value_multiplier = 1.0;
        for _ in 1..prev_word.chars().count() {
            prev_value_multiplier *= self.base;
            prev_value_multiplier %= self.modulus;
        }

        hash += self.modulus;
        hash -= (prev_value * prev_value_multiplier) % self.modulus;

        hash *= self.base;
        hash += new_value;
        hash %= self.modulus;

        hash
    }

    /// Converts char to number: its code point, and of one past the Basic
    /// Multilingual Plane, its low surrogate shifted past it too.
    pub fn char_to_number(&self, char: char) -> f64 {
        let mut units = [0; 2];
        let mut char_code = f64::from(u32::from(char));
        if let [_, surrogate] = char.encode_utf16(&mut units) {
            let surrogate_shift = 2f64.powi(16);
            char_code += f64::from(*surrogate) * surrogate_shift;
        }
        char_code
    }
}

js::export_default!(PolynomialHash);
