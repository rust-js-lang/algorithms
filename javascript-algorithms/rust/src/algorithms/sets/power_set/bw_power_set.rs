use js::Unknown;

/// Find power-set of a set using BITWISE approach.
pub fn bw_power_set<'a>(original_set: &[&'a Unknown]) -> Vec<Vec<&'a Unknown>> {
    let mut sub_sets = Vec::new();

    // We will have 2^n possible combinations (where n is a length of original set).
    // It is because for every element of original set we will decide whether to include
    // it or not (2 options for each set element).
    let number_of_combinations = 1 << original_set.len();

    // Each number in binary representation in a range from 0 to 2^n does exactly what we need:
    // it shows by its bits (0 or 1) whether to include related element from the set or not.
    // For example, for the set {1, 2, 3} the binary number of 0b010 would mean that we need to
    // include only "2" to the current set.
    for combination_index in 0..number_of_combinations {
        let mut sub_set = Vec::new();

        for (set_element_index, &element) in original_set.iter().enumerate() {
            // Decide whether we need to include current element into the subset or not.
            if combination_index & (1 << set_element_index) != 0 {
                sub_set.push(element);
            }
        }

        // Add current subset to the list of all subsets.
        sub_sets.push(sub_set);
    }

    sub_sets
}

js::export_default!(bw_power_set);
