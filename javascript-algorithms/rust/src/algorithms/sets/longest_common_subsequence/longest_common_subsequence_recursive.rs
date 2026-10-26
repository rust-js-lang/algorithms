use std::collections::HashMap;

/// The longest subsequence both strings have, recursively, each pair of
/// suffixes' answer kept in `memo`.
fn lcs(s1: &str, s2: &str, memo: &mut HashMap<String, String>) -> String {
    let (Some(first1), Some(first2)) = (s1.chars().next(), s2.chars().next()) else {
        return String::new();
    };

    let key = format!("{s1}:{s2}");
    if let Some(found) = memo.get(&key).filter(|found| !found.is_empty()) {
        return found.clone();
    }

    let (rest1, rest2) = (&s1[first1.len_utf8()..], &s2[first2.len_utf8()..]);
    if first1 == first2 {
        return format!("{first1}{}", lcs(rest1, rest2, memo));
    }

    let next_lcs1 = lcs(rest1, s2, memo);
    let next_lcs2 = lcs(s1, rest2, memo);

    let next_longest = if next_lcs1.len() >= next_lcs2.len() {
        next_lcs1
    } else {
        next_lcs2
    };

    memo.insert(key, next_longest.clone());

    next_longest
}

/// Longest Common Subsequence (LCS) (Recursive Approach).
pub fn longest_common_subsequence_recursive(string1: &str, string2: &str) -> String {
    lcs(string1, string2, &mut HashMap::new())
}

js::export_default!(longest_common_subsequence_recursive);
