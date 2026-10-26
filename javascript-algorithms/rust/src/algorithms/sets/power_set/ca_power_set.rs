use js::Unknown;

/// Find power-set of a set using CASCADING approach.
pub fn ca_power_set<'a>(original_set: &[&'a Unknown]) -> Vec<Vec<&'a Unknown>> {
    // Let's start with an empty set.
    let mut sets = vec![Vec::new()];

    // Now, let's say:
    // originalSet = [1, 2, 3].
    //
    // Let's add the first element from the originalSet to all existing sets:
    // [[]] ← 1 = [[], [1]]
    //
    // Adding the 2nd element to all existing sets:
    // [[], [1]] ← 2 = [[], [1], [2], [1, 2]]
    //
    // Adding the 3nd element to all existing sets:
    // [[], [1], [2], [1, 2]] ← 3 = [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]
    //
    // And so on for the rest of the elements from originalSet.
    // On every iteration the number of sets is doubled, so we'll get 2^n sets.
    for &element in original_set {
        let existing_sets_num = sets.len();

        for set_idx in 0..existing_sets_num {
            let set = [sets[set_idx].as_slice(), &[element]].concat();
            sets.push(set);
        }
    }

    sets
}

js::export_default!(ca_power_set);
