const DEFAULT_BASE: f64 = 17.0;

/// A polynomial hash of words' UTF-16 units, its modulus none.
#[cfg_attr(rust_js, rust_js::class)]
pub struct SimplePolynomialHash {
    pub base: f64,
}

unsafe impl js::Defined for SimplePolynomialHash {}

#[cfg_attr(rust_js, rust_js::class)]
impl SimplePolynomialHash {
    /// @param {number} [base] - Base number that is used to create the polynomial.
    pub fn new(base: Option<f64>) -> Self {
        SimplePolynomialHash {
            base: base.unwrap_or(DEFAULT_BASE),
        }
    }

    /// Function that creates hash representation of the word.
    ///
    /// Time complexity: O(word.length).
    ///
    /// @assumption: This version of the function doesn't use modulo operator.
    /// Thus it may produce number overflows by generating numbers that are
    /// bigger than Number.MAX_SAFE_INTEGER. This function is mentioned here
    /// for simplicity and LEARNING reasons.
    pub fn hash(&self, word: &str) -> f64 {
        let mut hash = 0.0;
        for (char_index, char_code) in word.encode_utf16().enumerate() {
            hash += f64::from(char_code) * self.base.powf(char_index as f64);
        }

        hash
    }

    /// Function that creates hash representation of the word
    /// based on previous word (shifted by one character left) hash value.
    ///
    /// Time complexity: O(1).
    pub fn roll(&self, prev_hash: f64, prev_word: &str, new_word: &str) -> f64 {
        let mut hash = prev_hash;

        let prev_value = prev_word.encode_utf16().next().map_or(f64::NAN, f64::from);
        let new_value = new_word.encode_utf16().last().map_or(f64::NAN, f64::from);

        hash -= prev_value;
        hash /= self.base;
        hash += new_value
            * self
                .base
                .powf((new_word.encode_utf16().count() as f64) - 1.0);

        hash
    }
}

js::export_default!(SimplePolynomialHash);
