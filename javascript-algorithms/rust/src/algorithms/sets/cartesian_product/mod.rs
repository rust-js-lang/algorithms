use js::Unknown;

/// Generates Cartesian Product of two sets: each pair of an item of
/// `set_a` and one of `set_b`; `null` where either is empty or missing.
#[cfg_attr(rust_js, rust_js::nullable)]
pub fn cartesian_product<'a>(
    set_a: Option<&[&'a Unknown]>,
    set_b: Option<&[&'a Unknown]>,
) -> Option<Vec<(&'a Unknown, &'a Unknown)>> {
    // Check if input sets are not empty.
    // Otherwise return null since we can't generate Cartesian Product out of them.
    let (Some(set_a), Some(set_b)) = (set_a, set_b) else {
        return None;
    };
    if set_a.is_empty() || set_b.is_empty() {
        return None;
    }

    // Init product set.
    let mut product = Vec::new();

    // Now, let's go through all elements of a first and second set and form all possible pairs.
    for &a in set_a {
        for &b in set_b {
            // Add current product pair to the product set.
            product.push((a, b));
        }
    }

    // Return cartesian product set.
    Some(product)
}

js::export_default!(cartesian_product);
