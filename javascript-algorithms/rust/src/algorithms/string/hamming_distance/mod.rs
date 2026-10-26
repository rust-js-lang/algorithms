/// How many characters of `a` and `b`, of one length, differ.
pub fn hamming_distance(a: &str, b: &str) -> usize {
    if a.len() != b.len() {
        panic!("Strings must be of the same length");
    }

    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

js::export_default!(hamming_distance);
