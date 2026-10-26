/// Whether `string` reads the same backwards, from both ends to the middle.
pub fn is_palindrome(string: &str) -> bool {
    let mut chars = string.chars();
    while let (Some(left), Some(right)) = (chars.next(), chars.next_back()) {
        if left != right {
            return false;
        }
    }
    true
}

js::export_default!(is_palindrome);
